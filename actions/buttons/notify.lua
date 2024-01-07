notify = require("notify")

mod = {
    name = "Send Notify"
}

mod.on_click = function()
    notify:new()
        :summary("Testing")
        :show()
    print("clicked")
end

mod.on_long_click = function()
    print("long clicked")
end

mod.get_icon = function()
    -- return {
    --     path = "notify.png"
    -- }
    -- return {
    --     data =
    --     "base64 encoded svg png or elsewere"
    --     ,
    --     content_type = "image/svg"
    -- }
    -- return {
    --     text = "ON!"
    -- }
end

return mod
