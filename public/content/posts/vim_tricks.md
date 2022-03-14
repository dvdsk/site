---
title: "Vim recipes"
date: 2022-03-12T10:15:30+01:00
draft: true
---

My list of intermediate Neovim recipes, not so obvious operations and functions that make tasks easier and safer. This is a living document and growing as I find more good patterns.


## Quick fix list fun
Find a text pattern and execute vim grammar on all lines that match. This one is especially useful wile refactoring, for example removing an argument from a function. By using telescope we can quickly find the pattern that matches only the lines we want to change

1. Find all instances you want to change using [telescopes's](https://github.com/nvim-telescope/telescope.nvim) live regex `builtin.live_grep`. It will search through all not git-ignored files in vims current working directory.
2. Send the entries to the _quick fix list_ `Ctrl + q`
3. Quick fix lists just openend. Go to the first item (press `enter`) or use `:cfirst`. 
4. Record a macro (I press q twice, recording in register q). Thanks the to previous step we start with the cursor at the exact place it will be when later executing the macro.
5. Undo the changes done while recording the macro (or they will be applied twice)
5. Use the magic: `:cdo norm! @q`. Also known as: quick fix do, go to normal mode, run macro in register q.

## Custom (tele)scope
LSP's are great for navigating through code. I constantly _go to definition_ or _find all occurences_ especially functions. Some languages have semantics that make it hard for an LSP (lua, python) or the LSP isnt quite there yet. I used live grep ([telescope](https://github.com/nvim-telescope/telescope.nvim)) instead searching for the name with a `(` attached. This small gist automates that and improves upon it by limiting the search to relevant files.

Support another language by adding a regex and valid file extensions. Or adapt it building a text search for another pattern limited.

To try it out put the code below in a .lua file and execute the file using `:luafile %`. If you like it add it to your config and bind a key to `function_scope()`:
```lua
local function_str = {
 ["lua"] = "function",
 ["c"] = ".+ .+\\(.*\\) \\{", -- word followed by a name followed by arguments
}

local extensions = {
 ["lua"] = { "lua" },
 ["c"] = { "c", "h" },
}

local function function_scope()
    local bufnr = vim.api.nvim_get_current_buf()
    local filetype = vim.bo[bufnr].filetype         -- filetype for current buffer
    local search = function_str[filetype] .. " "    -- get the function pattern for this filetype
    local ext_patterns = function()                 -- filter candidate files by extension
        for _, ext in ipairs(extensions[filetype]) do  
            list[#list + 1] = "-g*." .. ext .. ""
        end
		return list
    end

    require('telescope.builtin').live_grep({
	  default_text = search,
	  prompt_title = "find function",
	  additional_args = patterns
    })
end
function_scope() -- call this from a keybind instead
```
_idea from: [u/Combinatorilliance](https://www.reddit.com/r/neovim/comments/st1kxs/some_telescope_tips/)_
