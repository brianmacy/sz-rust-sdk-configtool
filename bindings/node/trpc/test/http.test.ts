/** End-to-end over HTTP: standalone server + typed client, superjson, POST queries. */
import assert from "node:assert/strict";
import type { AddressInfo } from "node:net";
import { after, before, test } from "node:test";

import { createTRPCClient, httpLink, TRPCClientError } from "@trpc/client";
import { createHTTPServer } from "@trpc/server/adapters/standalone";
import superjson from "superjson";

import { configToolRouter, type ConfigToolRouter } from "../dist/index.js";
import { fixture } from "../../test/helpers.ts";

const server = createHTTPServer({ router: configToolRouter, allowMethodOverride: true });
let client: ReturnType<typeof createTRPCClient<ConfigToolRouter>>;

before(async () => {
  await new Promise<void>((resolve) => server.listen(0, resolve));
  const { port } = server.address() as AddressInfo;
  client = createTRPCClient<ConfigToolRouter>({
    links: [
      httpLink({ url: `http://127.0.0.1:${port}`, transformer: superjson, methodOverride: "POST" }),
    ],
  });
});

after(() => new Promise<void>((resolve) => server.close(() => resolve())));

test("mutation + POSTed query carry the full template config", async () => {
  assert.ok(fixture.length > 100_000);
  const cfg = await client.addDataSource.mutate({ config: fixture, code: "crm" });
  const list = await client.listDataSources.query({ config: cfg });
  assert.match(list, /"dataSource":"CRM"/);
});

test("typed outputs: named JSON texts and an int_or_str selector over HTTP", async () => {
  const v = await client.verifyCompatibilityVersion.query({ config: fixture, expectedVersion: "11" });
  assert.equal(v.currentVersion, '"11"');
  assert.equal(v.matches, "true");
  const byCode = await client.getComparisonCall.query({ config: fixture, call: "NAME" });
  assert.equal(byCode, await client.getComparisonCall.query({ config: fixture, call: 1 }));
});

test("library errors reach the client with data.szConfigTool", async () => {
  try {
    await client.getDataSource.query({ config: fixture, code: "NOPE" });
    assert.fail("expected a rejection");
  } catch (e) {
    assert.ok(e instanceof TRPCClientError);
    const data = e.data as { code: string; szConfigTool: { reasonCode: string; kind: string } };
    assert.equal(data.code, "NOT_FOUND");
    assert.equal(data.szConfigTool.reasonCode, "NOT_FOUND");
    assert.equal(data.szConfigTool.kind, "NOT_FOUND");
  }
});
