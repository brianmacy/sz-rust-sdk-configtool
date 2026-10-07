// szconfigtool.hpp -- header-only C++20 binding of libSzConfigTool.
//
// Stateless functions over an opaque configuration JSON string:
//
//   #include <szconfigtool/szconfigtool.hpp>
//   std::string cfg = szconfigtool::AddDataSource(template_json, "CRM");
//   std::string ds  = szconfigtool::GetDataSource(cfg, "CRM");  // JSON text
//
// Every function throws szconfigtool::SzConfigToolException on failure.
#ifndef SZCONFIGTOOL_SZCONFIGTOOL_HPP
#define SZCONFIGTOOL_SZCONFIGTOOL_HPP

#include "szconfigtool/core.hpp"
#include "szconfigtool/generated/api.hpp"

#endif  // SZCONFIGTOOL_SZCONFIGTOOL_HPP
