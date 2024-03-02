local notify = require("notify")
local mod = {
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
    return {
        type = "url",
        url = "https://api.picup.click/cdn/55383857-37da-412b-bd61-3da90366ef35/01HM0ZS2PE7DWDK29VYRKC58RQ.png"
    }
end


return mod
