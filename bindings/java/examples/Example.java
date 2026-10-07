import io.github.brianmacy.szconfigtool.ConfigAndJson;
import io.github.brianmacy.szconfigtool.SzConfigTool;
import io.github.brianmacy.szconfigtool.SzConfigToolException;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * Add a data source and an attribute to a Senzing configuration file.
 *
 * <p>Run: {@code java -cp sz-configtool-<version>.jar Example.java <g2config.json>}
 */
public final class Example {
    public static void main(String[] args) throws Exception {
        if (args.length != 1) {
            System.err.println("usage: java -cp sz-configtool.jar Example.java <g2config.json>");
            System.exit(2);
        }
        String config = Files.readString(Path.of(args[0]));

        config = SzConfigTool.addDataSource(config, "CUSTOMERS");
        ConfigAndJson added = SzConfigTool.addAttribute(config, "CUST_NAME", "NAME", "FULL_NAME",
                "NAME", new SzConfigTool.AddAttributeOptions().internal("No"));
        config = added.config();
        System.out.println("new attribute row: " + added.json());
        System.out.println("data source: " + SzConfigTool.getDataSource(config, "CUSTOMERS"));

        try {
            SzConfigTool.addDataSource(config, "CUSTOMERS");
        } catch (SzConfigToolException e) {
            System.out.println("expected failure: " + e.getReasonCode() + " - " + e.getMessage());
        }
    }
}
