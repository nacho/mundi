install_data('io.github.nacho.mundi.gschema.xml',
  install_dir: get_option('datadir') / 'glib-2.0' / 'schemas'
)

install_data(
  'icons' / 'io.github.nacho.mundi.svg',
  install_dir: get_option('datadir') / 'icons' / 'hicolor' / 'scalable' / 'apps'
)

appstream_file = files('io.github.nacho.mundi.metainfo.xml')
install_data(appstream_file,
  install_dir: get_option('datadir') / 'metainfo'
)

desktop_file = i18n.merge_file(
  input: 'io.github.nacho.mundi.desktop.in',
  output: 'io.github.nacho.mundi.desktop',
  type: 'desktop',
  po_dir: meson.project_source_root() / 'po',
  install: true,
  install_dir: get_option('datadir') / 'applications'
)

gnome.compile_schemas(build_by_default: true)

desktop_utils = find_program('desktop-file-validate', required: false, disabler: true)
test(
  'Validate desktop file',
   desktop_utils,
   args: [desktop_file]
)

appstreamcli = find_program('appstreamcli', required: false, disabler: true)
test(
  'Validate appstream file',
   appstreamcli,
   args: ['validate', '--no-net', '--explain', appstream_file]
)

compile_schemas = find_program('glib-compile-schemas', required: false, disabler: true)
test(
  'Validate schema file',
   compile_schemas,
   args: ['--strict', '--dry-run', meson.current_source_dir()]
)