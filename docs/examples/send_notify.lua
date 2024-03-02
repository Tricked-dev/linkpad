local notify = require("notify")

local mod = {
    name = "Send Notify",
    on_click = function()
        notify:new()
            :summary("Testing")
            :show()
        print("clicked")
    end
}

return mod
