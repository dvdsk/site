---
title: "Vim recipes"
date: 2022-03-12T10:15:30+01:00
draft: true
---

## Quick fix list fun


1. Find all instances you want to change using (telescopes's)[https://github.com/nvim-telescope/telescope.nvim] live regex `builtin.live_grep`. It will search through all not git-ignored files in vims current working directory.
2. Send the entries to the _quick fix list_ `Ctrl + q`
3. Quick fix lists just openend. Go to the first item (press `enter`) or use `:cfirst`
4. Record a macro. I just press q twice recording in register q.
5. Use the magic: `:cdo norm! @q`. Also known as: quick fix do, go to normal mode, run macro in register q.
