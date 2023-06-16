local dummy = require("hua:dummy")
local json = require("json")

print(dummy.version())

print(json.encode({
  c = 5
}))