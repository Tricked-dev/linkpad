local ev = require("event_sender")
local mod = {
    name = "Switch scene"
}

local last = "left"

mod.on_click = function()
    if last == "left" then
        last = "right"
        ev.keybind("ctrl + alt + shift + L")
    else
        ev.keybind("ctrl + alt + shift + P")
        last = "left"
    end
end

return mod
