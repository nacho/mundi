install_data('io.github.nacho.mundi.gschema.xml',
  install_dir: get_option('datadir') / 'glib-2.0' / 'schemas'
)

install_data(
  'icons' / 'io.github.nacho.mundi.svg',
  install_dir: get_option('datadir') / 'icons' / 'hicolor' / 'scalable' / 'apps'
)

install_data('io.github.nacho.mundi.metainfo.xml',
  install_dir: get_option('datadir') / 'metainfo'
)

i18n.merge_file(
  input: 'io.github.nacho.mundi.desktop.in',
  output: 'io.github.nacho.mundi.desktop',
  type: 'desktop',
  po_dir: meson.project_source_root() / 'po',
  install: true,
  install_dir: get_option('datadir') / 'applications'
)

gnome.compile_schemas(build_by_default: true)
