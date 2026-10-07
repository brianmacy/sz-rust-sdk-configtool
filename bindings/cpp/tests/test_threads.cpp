// The C library's last-error slot is thread-local; the binding reads it on
// the calling thread right after the call, so concurrent failures never leak
// across threads.
#include <gtest/gtest.h>

#include <atomic>
#include <barrier>
#include <string>
#include <thread>
#include <vector>

#include "conformance_support.hpp"
#include "generated/typed_dispatch.hpp"

namespace szconfigtool_test {
namespace {

namespace sz = szconfigtool;
using sz::ErrorKind;

std::string LoadFixture() {
    const std::string conf = ReadFile(WorkspacePath(kConformanceJson));
    return ReadFile(WorkspacePath(json::Parse(conf).Find("fixture")->text));
}

/// One thread's workload: each call must fail (or succeed) with ITS kind.
ErrorKind Attempt(int role, const std::string& fixture) {
    try {
        switch (role % 3) {
            case 0: (void)sz::GetDataSource(fixture, "NOPE"); break;
            case 1: (void)sz::ListDataSources("not json"); break;
            default: (void)sz::GetDataSource(fixture, "TEST"); return ErrorKind::Unknown;
        }
    } catch (const sz::SzConfigToolException& e) {
        return e.Kind();
    }
    return ErrorKind::Unknown;
}

constexpr ErrorKind kExpected[] = {ErrorKind::NotFound, ErrorKind::JsonParse, ErrorKind::Unknown};

TEST(Threads, ErrorsAreIsolatedPerThread) {
    const std::string fixture = LoadFixture();
    constexpr int kThreads = 9;
    constexpr int kIterations = 25;
    std::barrier sync(kThreads);
    std::atomic<int> mismatches{0};
    std::vector<std::thread> pool;
    for (int t = 0; t < kThreads; ++t) {
        pool.emplace_back([&, t] {
            sync.arrive_and_wait();
            for (int i = 0; i < kIterations; ++i) {
                if (Attempt(t, fixture) != kExpected[t % 3]) {
                    ++mismatches;
                }
            }
        });
    }
    for (auto& th : pool) {
        th.join();
    }
    EXPECT_EQ(mismatches.load(), 0);
}

TEST(Threads, AnotherThreadsFailureIsInvisible) {
    const std::string fixture = LoadFixture();
    EXPECT_EQ(Attempt(0, fixture), ErrorKind::NotFound);
    const char* seen_elsewhere = "unset";
    std::thread([&] { seen_elsewhere = SzConfigTool_getLastErrorReasonCode(); }).join();
    EXPECT_EQ(seen_elsewhere, nullptr);
    ASSERT_NE(SzConfigTool_getLastErrorReasonCode(), nullptr);
    EXPECT_STREQ(SzConfigTool_getLastErrorReasonCode(), "NOT_FOUND");
}

}  // namespace
}  // namespace szconfigtool_test
