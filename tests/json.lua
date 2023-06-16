local json = require("hua:json")

local mod = {}

function mod.encode(value)
  return json.encode(value)
end

return mod