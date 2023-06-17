local dummy = require("hua:dummy")
local json = require("hua:json")

print(dummy.version())

local text = {
  ["moom"] = "x",
  ["lmaop"] = "e"
}

print(json.encode(text))