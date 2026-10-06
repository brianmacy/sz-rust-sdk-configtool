// quickstart.cpp -- add, read and update a data source in a Senzing config.
//
// Usage: szconfigtool_quickstart <path to g2config.json>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

#include <szconfigtool/szconfigtool.hpp>

int main(int argc, char** argv) {
    if (argc != 2) {
        std::cerr << "usage: " << argv[0] << " <g2config.json>\n";
        return 2;
    }
    std::ifstream in(argv[1], std::ios::binary);
    if (!in) {
        std::cerr << "cannot read " << argv[1] << "\n";
        return 2;
    }
    std::stringstream buf;
    buf << in.rdbuf();
    const std::string config = buf.str();

    namespace sz = szconfigtool;
    try {
        // Optional args: designated initializers on the generated options struct.
        std::string updated = sz::AddDataSource(config, "CUSTOMERS", {.id = 5001});
        std::cout << "added:   " << sz::GetDataSource(updated, "CUSTOMERS") << "\n";

        // Tri-state update: Leave (default) / Clear / Set.
        updated = sz::SetFragment(updated, "SNAME_SSTAB",
                                  {.description = sz::FieldUpdate<std::string>::Clear()});
        std::cout << "fragment: " << sz::GetFragment(updated, "SNAME_SSTAB") << "\n";

        (void)sz::GetDataSource(updated, "NO_SUCH_SOURCE");
        std::cerr << "expected NOT_FOUND\n";
        return 1;
    } catch (const sz::SzConfigToolException& e) {
        if (e.Kind() != sz::ErrorKind::NotFound) {
            std::cerr << e.ReasonCode() << ": " << e.what() << "\n";
            return 1;
        }
        std::cout << "error:   " << e.ReasonCode() << " (" << e.what() << ")\n";
    }
    std::cout << "libSzConfigTool " << sz::LibraryVersion() << "\n";
    return 0;
}
