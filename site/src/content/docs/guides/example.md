---
title: Example Reference
description: A reference page in my new Starlight docs site.
---

= Examples

Tricked <tricked@tricked.dev>

Here is a example module that just sends a notification when clicked, it uses the build in notify module to achieve this.

---

```lua file=<rootDir>/send_notify.lua title=send_notify.lua

```

You can also run keybinds using the `event_sender` module

```lua file=<rootDir>/switch_scene.lua title=switch_scene.lua {11} {13}

```

You can bind keybinds scenes to keybinds so in this example i bound the first scene to `ctrl+alt+shift+l` and the other to `P`, once the button is clicked it'll check what the last one was and do the other one making the scenes switch.

You can bind scenes in obs in the keybind menu and searching scene.
