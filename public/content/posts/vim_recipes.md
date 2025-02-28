---
title: "Vim recipes"
date: 2022-03-20T10:12:30+01:00
draft: false
---

My list of intermediate _Neovim_ recipes, sequences of user actions and lua functions that make repetative tasks easier and safer. This is a living document and growing as I find more good patterns.


## Quick fix list fun
Find a text pattern and execute vim grammar on all lines that match. This one is especially useful wile refactoring, for example removing an argument from a function. By using telescope we can quickly find the pattern that matches only the lines we want to change

1. Find all instances you want to change using [telescopes's](https://github.com/nvim-telescope/telescope.nvim) live regex `builtin.live_grep`. It will search through all not git-ignored files in vims current working directory.
2. Send the entries to the _quick fix list_ `Ctrl + q`
3. Quick fix lists just openend. Go to the first item (press `enter`) or use `:cfirst`. 
4. Record a macro (I press q twice, recording in register q). Thanks the to previous step we start with the cursor at the exact place it will be when later executing the macro.
5. Undo the changes done while you where recording the macro (or they will be applied twice)
5. Use the magic: `:cdo norm! @q`. Also known as: quick fix do, go to normal mode, run macro in register q.

tip: (temporarily) set your auto-formatter to never break lines and reformat your
codebase. For rust format: add `max_width=400` and `chain_width=200` to
`rustfmt.toml`.

## Custom (tele)scope
LSP's are great for navigating through code. I constantly use _go to definition_ and _lsp_references_ especially for locating functions. Some languages (lua, python) have semantics that make it hard for an LSP to find these or the LSP isnt quite there yet. I used to open live grep ([telescope](https://github.com/nvim-telescope/telescope.nvim)) and search for the function name name with a `(` attached. This small gist automates and improves upon that by limiting the search to relevant files and picking the function pattern automatically.

Support another language by adding a regex and valid file extensions. Or adapt it to build a text search for another pattern.

```lua
local function_def = {
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
    local search = function_def[filetype] .. " "    -- get the function pattern for this filetype
    local ext_patterns = function()                 -- filter candidate files by extension
        local list = {}
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

To try it out put the code below in a `lua` file and execute the file using `:luafile %`. If you like the idea take a look at the [complete code](https://github.com/dvdsk/new-linux-setup/blob/master/vim/lua/functions.lua) and map it using: 
```lua
map("n", "<leader>u", ":lua require'functions'.func_def_scope()<CR>", silent)
```

_idea from: [u/Combinatorilliance](https://www.reddit.com/r/neovim/comments/st1kxs/some_telescope_tips/)_
