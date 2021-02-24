" Specify a directory for plugins (for Neovim: ~/.local/share/nvim/plugged)
call plug#begin('~/.vim/plugged')

" Make sure you use single quotes
Plug 'tpope/vim-fugitive'
Plug 'tpope/vim-surround'
Plug 'tpope/vim-commentary'
Plug 'altercation/vim-colors-solarized'
Plug 'bling/vim-airline'
Plug 'vim-airline/vim-airline-themes'
Plug 'scrooloose/nerdtree'
Plug 'scrooloose/nerdcommenter'
Plug 'ervandew/supertab'
Plug 'raimondi/delimitmate'
Plug 'godlygeek/tabular'
Plug 'elzr/vim-json'
Plug 'davidhalter/jedi-vim'
Plug 'greyblake/vim-preview'
Plug 'tmhedberg/SimpylFold'
Plug 'lervag/vimtex'
Plug 'tell-k/vim-autopep8'
Plug 'kien/ctrlp.vim'
Plug 'nvie/vim-flake8'
Plug 'JamshedVesuna/vim-markdown-preview'

" Shorthand notation; fetches https://github.com/junegunn/vim-easy-align
Plug 'junegunn/vim-easy-align'

" Any valid git URL is allowed
" Plug 'https://github.com/junegunn/vim-github-dashboard.git'

" Multiple Plug commands can be written in a single line using | separators
" Plug 'SirVer/ultisnips' | Plug 'honza/vim-snippets'

" On-demand loading
" Plug 'scrooloose/nerdtree', { 'on':  'NERDTreeToggle' }
" Plug 'tpope/vim-fireplace', { 'for': 'clojure' }

" Using a non-master branch
" Plug 'rdnetto/YCM-Generator', { 'branch': 'stable' }

" Using a tagged release; wildcard allowed (requires git 1.9.2 or above)
" Plug 'fatih/vim-go', { 'tag': '*' }

" Plugin options
" Plug 'nsf/gocode', { 'tag': 'v.20150303', 'rtp': 'vim' }

" Plugin outside ~/.vim/plugged with post-update hook
Plug 'junegunn/fzf', { 'dir': '~/.fzf', 'do': './install --all' }
Plug 'junegunn/fzf.vim'

" Unmanaged plugin (manually installed and updated)
" Plug '~/my-prototype-plugin'

" Initialize plugin system
call plug#end()

let mapleader="\\"
set autoread
syntax on
set nowrap
set tabstop=4
set expandtab
set autoindent
set incsearch
set nu
set colorcolumn=80
highlight ColorColumn ctermbg=Black
set showmatch
set ignorecase
set tw=80
let python_highlight_all=1

" Folding
inoremap <F9> <C-O>za
nnoremap <F9> za
onoremap <F9> <C-C>za
vnoremap <F9> zf

" Window navigation
nnoremap <C-h> <C-w>h
        nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l

" New empty buffer
nnoremap <C-n> :enew

" Markdown preview
let vim_markdown_preview_github=1
let vim_markdown_preview_browser='Firefox'
let vim_markdown_preview_toggle=2

" NERDTree
let NERDTreeShowHidden=1
" autocmd VimEnter * NERDTree | wincmd p
autocmd bufenter * if (winnr("$") == 1 && exists("b:NERDTree") && b:NERDTree.isTabTree()) | q | endif
let g:NERDTreeDirArrowExpandable = '▸'
let g:NERDTreeDirArrowCollapsible = '▾'
map <Leader>p :NERDTreeToggle<CR>
" No file
autocmd StdinReadPre * let s:std_in=1
autocmd VimEnter * if argc() == 0 && !exists("s:std_in") | NERDTree | endif
" Directory
autocmd StdinReadPre * let s:std_in=1
autocmd VimEnter * if argc() == 1 && isdirectory(argv()[0]) && !exists("s:std_in") | exe 'NERDTree' argv()[0] | wincmd p | ene | endif

"NERDCommenter
let g:NERDSpaceDelims = 1
let g:NERDAltDelims_python = 1
let g:NERDCompactSexyComs = 1
let g:NERDTrimTrailingWhitespace = 1

" Python Mode Plugin
let pymode=1

" Flake8
let g:flake8_show_in_gutter=1

" SimplyFold
autocmd BufWinEnter *.py setlocal foldexpr=SimpylFold(v:lnum) foldmethod=expr
autocmd BufWinLeave *.py setlocal foldexpr< foldmethod<
let g:SimpylFold_fold_docstring = 0
let g:SimpylFold_fold_import = 0

" FZF
set rtp+=/usr/local/opt/fzf

" CTRL-P
noremap <C-a> :CtrlP /Users/matthewmcguire/ProtectWise<CR>

"" PowerLine
"set rtp+=/usr/local/lib/python2.7/dist-packages/powerline/bindings/vim/
"" Always show statusline
"set laststatus=2
"" Use 256 colours (Use this setting only if your terminal supports 256 colours)

" Vim Airline
set laststatus=2
let g:airline_powerline_fonts = 0
let g:airline_theme='powerlineish'
let g:airline#extensions#bufferline#enabled = 1
let g:airline#extensions#tabline#enabled = 1
let g:airline#extensions#tabline#show_buffers = 1
let g:airline#extensions#tabline#buffer_nr_show = 1
set t_Co=256

" Vim Preview
let g:PreviewBrowsers='firefox,chrome'
:nmap <Leader>m :Preview<CR>

" Use system clipboard for Copy and Paste
noremap <Leader>y "+y
noremap <Leader>P "+p

" NEW FIND FUNCTION DEFINITION
" https://medium.com/@crashybang/supercharge-vim-with-fzf-and-ripgrep-d4661fc853d2
" --column: Show column number
" --line-number: Show line number
" --no-heading: Do not show file headings in results
" --fixed-strings: Search term as a literal string
" --ignore-case: Case insensitive search
" --no-ignore: Do not respect .gitignore, etc...
" --hidden: Search hidden files and folders
" --follow: Follow symlinks
" --glob: Additional conditions for search (in this case ignore everything in the .git/ folder)
" --color: Search color options

command! -bang -nargs=* Find call fzf#vim#grep('rg --column --line-number --no-heading --fixed-strings --ignore-case --no-ignore --hidden --follow --glob "!.git/*" --color "always" '.shellescape(<q-args>), 1, <bang>0)
