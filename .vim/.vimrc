set hlsearch		" Highlight searched words.
set incsearch		" Inclemental search.
set laststatus=2	" Print file name always.
set nf=alpha,hex	" Inclement alphabet by [Ctrl-a].
set number		" Print line number.
set paste		" Paste literally.
set relativenumber	" Print cursor relative line number.
set ruler		" Print cursor position.
set showcmd		" Show command being entered.

" Expand a tab into 4 spaces only when editing rust source file.
filetype plugin on
filetype indent on
autocmd FileType rust setlocal expandtab shiftwidth=4 softtabstop=4 tabstop=4

