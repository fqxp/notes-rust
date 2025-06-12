# notes-rust

Currently a rust learning project

To run it, use `cargo run`.

# Development

## Design Goals

To develop a note-taking tool that:

- is easy to learn and use
- is using open data formats
- is encourages using complement tools to modify notes
- is pleasant to use

## Milestone: Basic note editing

- 0.1: basic functionality
  [x] basic, flat note list
  [x] basic note view
  [x] filter note list by title
  [x] change sort order
  [x] navigating directory tree
  [x] display further info on individual notes (e.g., file name, size on disk, file type)
  [x] About dialog
  [ ] proper error reporting
  [ ] basic attachment view (e.g. images, videos, button for opening with external app)
  [ ] breadcrumbs for current folder widget
  [ ] create new notes
  [ ] create new folders
  [ ] add new "notebook" (naming?)
- 0.2: UX
  [ ] CSS layout for note view (e.g. thomasf-solarizedcsslight)
  [ ] UI: note list
  [ ] copy&paste notes and folders
- 0.3: settings
  [ ] extra tab?

## Backlog (unsorted)

- multi-window
  [ ] multi-window functionality
- wiki functionality
  [ ] display missing links in red (Mediawiki-like)
  [ ] create new note by clicking on red link
  [ ] use index.md as default note for folders
- export
  [ ] export to HTML
  [ ] export to PDF
  [ ] export to ODT
  [ ] export to LaTeX
  [ ] custom export
- misc features
  [ ] asciidoc support
  [ ] frontmatter support
  [ ] HTML support
  [ ] drag&drop notes -> folders
  [ ] drag&drop files from external sources
- attachments
  [ ] mark missing attachments as red links or placeholders
  [ ] drag&drop file into note or note list
- per-notebook settings
  [ ] stylesheet
- basic sync tool support
  [ ] show sync info: which tool (syncthing, nextcloud, git)
  [ ] sync info: conflicts
- tool support
  [ ] hugo
  [ ] mdbook
  [ ] jupyter?
  [ ] org-mode
  [ ] slides with present/presenterm/pandoc
- other storage options
  [ ] WebDAV
  [ ] Nextcloud?
  [ ] Google Drive?
  [ ]
- extras
  [ ] rofi-notes
  [ ] dmenu-notes
  [ ] clipboard-to-note
  [ ] CLI tool
