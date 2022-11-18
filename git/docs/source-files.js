var sourcesIndex = JSON.parse('{\
"cairo":["",[["font",[],["font_extents.rs","font_face.rs","font_options.rs","glyph.rs","mod.rs","scaled_font.rs","text_cluster.rs","text_extents.rs","user_fonts.rs"]]],["constants.rs","context.rs","device.rs","enums.rs","error.rs","image_surface.rs","lib.rs","matrices.rs","paths.rs","patterns.rs","pdf.rs","ps.rs","quartz_surface.rs","recording_surface.rs","rectangle.rs","rectangle_int.rs","region.rs","stream.rs","surface.rs","surface_macros.rs","surface_png.rs","svg.rs","user_data.rs","utils.rs","win32_surface.rs","xcb.rs"]],\
"cairo_sys":["",[],["gobject.rs","lib.rs"]],\
"gdk_pixbuf":["",[["auto",[],["enums.rs","flags.rs","mod.rs","pixbuf.rs","pixbuf_animation.rs","pixbuf_format.rs","pixbuf_loader.rs","pixbuf_non_anim.rs","pixbuf_simple_anim.rs"]]],["lib.rs","pixbuf.rs","pixbuf_animation.rs","pixbuf_animation_iter.rs","prelude.rs"]],\
"gdk_pixbuf_sys":["",[],["lib.rs"]],\
"gio":["",[["auto",[],["action.rs","action_group.rs","action_map.rs","app_info.rs","app_info_monitor.rs","app_launch_context.rs","application.rs","application_command_line.rs","async_initable.rs","async_result.rs","buffered_input_stream.rs","buffered_output_stream.rs","bytes_icon.rs","cancellable.rs","charset_converter.rs","constants.rs","converter.rs","converter_input_stream.rs","converter_output_stream.rs","credentials.rs","data_input_stream.rs","data_output_stream.rs","dbus_action_group.rs","dbus_arg_info.rs","dbus_auth_observer.rs","dbus_connection.rs","dbus_interface.rs","dbus_interface_info.rs","dbus_interface_skeleton.rs","dbus_menu_model.rs","dbus_message.rs","dbus_method_info.rs","dbus_method_invocation.rs","dbus_node_info.rs","dbus_object.rs","dbus_property_info.rs","dbus_proxy.rs","dbus_server.rs","dbus_signal_info.rs","debug_controller.rs","debug_controller_dbus.rs","desktop_app_info.rs","drive.rs","emblem.rs","emblemed_icon.rs","enums.rs","file.rs","file_attribute_info_list.rs","file_attribute_matcher.rs","file_enumerator.rs","file_icon.rs","file_info.rs","file_input_stream.rs","file_io_stream.rs","file_monitor.rs","file_output_stream.rs","filename_completer.rs","filter_input_stream.rs","filter_output_stream.rs","flags.rs","functions.rs","icon.rs","inet_address.rs","inet_address_mask.rs","inet_socket_address.rs","initable.rs","input_stream.rs","io_stream.rs","list_model.rs","list_store.rs","loadable_icon.rs","memory_input_stream.rs","memory_monitor.rs","memory_output_stream.rs","menu.rs","menu_attribute_iter.rs","menu_item.rs","menu_link_iter.rs","menu_model.rs","mod.rs","mount.rs","mount_operation.rs","network_address.rs","network_monitor.rs","network_service.rs","notification.rs","output_stream.rs","permission.rs","pollable_input_stream.rs","pollable_output_stream.rs","power_profile_monitor.rs","property_action.rs","proxy.rs","proxy_address.rs","proxy_resolver.rs","remote_action_group.rs","resolver.rs","resource.rs","seekable.rs","settings.rs","settings_backend.rs","settings_schema.rs","settings_schema_key.rs","settings_schema_source.rs","simple_action.rs","simple_action_group.rs","simple_io_stream.rs","simple_permission.rs","simple_proxy_resolver.rs","socket.rs","socket_address.rs","socket_address_enumerator.rs","socket_client.rs","socket_connectable.rs","socket_connection.rs","socket_listener.rs","socket_service.rs","srv_target.rs","subprocess.rs","subprocess_launcher.rs","tcp_connection.rs","themed_icon.rs","threaded_socket_service.rs","tls_backend.rs","tls_certificate.rs","tls_client_connection.rs","tls_connection.rs","tls_database.rs","tls_file_database.rs","tls_interaction.rs","tls_password.rs","tls_server_connection.rs","unix_fd_list.rs","unix_input_stream.rs","unix_mount_entry.rs","unix_mount_point.rs","unix_output_stream.rs","unix_socket_address.rs","vfs.rs","volume.rs","volume_monitor.rs","zlib_compressor.rs","zlib_decompressor.rs"]],["subclass",[],["action_group.rs","action_map.rs","application.rs","async_initable.rs","initable.rs","input_stream.rs","io_stream.rs","list_model.rs","mod.rs","output_stream.rs","seekable.rs"]]],["action_entry.rs","action_map.rs","app_info.rs","application.rs","async_initable.rs","cancellable.rs","cancellable_future.rs","converter.rs","data_input_stream.rs","dbus.rs","dbus_connection.rs","dbus_message.rs","dbus_method_invocation.rs","dbus_node_info.rs","dbus_proxy.rs","debug_controller_dbus.rs","desktop_app_info.rs","error.rs","file.rs","file_attribute_info.rs","file_attribute_info_list.rs","file_attribute_matcher.rs","file_enumerator.rs","file_info.rs","flags.rs","gio_future.rs","inet_address.rs","inet_socket_address.rs","initable.rs","input_stream.rs","io_extension.rs","io_extension_point.rs","io_stream.rs","lib.rs","list_model.rs","list_store.rs","output_stream.rs","pollable_input_stream.rs","pollable_output_stream.rs","prelude.rs","read_input_stream.rs","resource.rs","settings.rs","simple_proxy_resolver.rs","socket.rs","subprocess.rs","subprocess_launcher.rs","task.rs","threaded_socket_service.rs","tls_connection.rs","unix_fd_list.rs","unix_input_stream.rs","unix_mount_entry.rs","unix_mount_point.rs","unix_output_stream.rs","unix_socket_address.rs","write_output_stream.rs"]],\
"gio_sys":["",[],["lib.rs","manual.rs"]],\
"glib":["",[["auto",[],["alias.rs","checksum.rs","constants.rs","date_time.rs","enums.rs","flags.rs","functions.rs","key_file.rs","main_context.rs","main_loop.rs","markup_parse_context.rs","mod.rs","source.rs","time_zone.rs","uri.rs"]],["gobject",[["auto",[],["binding.rs","binding_group.rs","flags.rs","mod.rs","signal_group.rs"]]],["binding.rs","binding_group.rs","flags.rs","mod.rs","signal_group.rs"]],["subclass",[],["basic.rs","boxed.rs","interface.rs","mod.rs","object.rs","object_impl_ref.rs","shared.rs","signal.rs","types.rs"]]],["boxed.rs","boxed_any_object.rs","boxed_inline.rs","bridged_logging.rs","byte_array.rs","bytes.rs","char.rs","checksum.rs","clone.rs","closure.rs","collections.rs","convert.rs","date.rs","date_time.rs","enums.rs","error.rs","functions.rs","future_with_timeout.rs","gstring.rs","gstring_builder.rs","key_file.rs","lib.rs","log.rs","main_context.rs","main_context_channel.rs","main_context_futures.rs","object.rs","param_spec.rs","prelude.rs","quark.rs","shared.rs","signal.rs","source.rs","source_futures.rs","thread_guard.rs","thread_pool.rs","time_span.rs","translate.rs","types.rs","unicollate.rs","utils.rs","value.rs","value_array.rs","variant.rs","variant_dict.rs","variant_iter.rs","variant_type.rs","wrapper.rs"]],\
"glib_build_tools":["",[],["lib.rs"]],\
"glib_macros":["",[["downgrade_derive",[],["enums.rs","fields.rs","mod.rs","structs.rs"]]],["boxed_derive.rs","clone.rs","closure.rs","enum_derive.rs","error_domain_derive.rs","flags_attribute.rs","lib.rs","object_interface_attribute.rs","object_subclass_attribute.rs","shared_boxed_derive.rs","utils.rs","variant_derive.rs"]],\
"glib_sys":["",[],["lib.rs","manual.rs"]],\
"gobject_sys":["",[],["lib.rs"]],\
"graphene":["",[["auto",[],["box_.rs","enums.rs","euler.rs","frustum.rs","matrix.rs","mod.rs","plane.rs","point.rs","point3_d.rs","quad.rs","quaternion.rs","ray.rs","rect.rs","size.rs","sphere.rs","triangle.rs","vec2.rs","vec3.rs","vec4.rs"]]],["box_.rs","euler.rs","frustum.rs","lib.rs","matrix.rs","plane.rs","point.rs","point3_d.rs","prelude.rs","quad.rs","quaternion.rs","ray.rs","rect.rs","size.rs","sphere.rs","triangle.rs","vec2.rs","vec3.rs","vec4.rs"]],\
"graphene_sys":["",[],["lib.rs"]],\
"pango":["",[["auto",[],["alias.rs","attr_list.rs","attribute.rs","color.rs","context.rs","enums.rs","flags.rs","font.rs","font_description.rs","font_face.rs","font_family.rs","font_map.rs","font_metrics.rs","fontset.rs","fontset_simple.rs","functions.rs","glyph_item.rs","glyph_string.rs","item.rs","language.rs","layout.rs","layout_iter.rs","layout_line.rs","matrix.rs","mod.rs","renderer.rs","tab_array.rs"]]],["analysis.rs","attr_class.rs","attr_color.rs","attr_float.rs","attr_font_desc.rs","attr_font_features.rs","attr_int.rs","attr_iterator.rs","attr_language.rs","attr_list.rs","attr_shape.rs","attr_size.rs","attr_string.rs","attribute.rs","color.rs","coverage.rs","enums.rs","functions.rs","glyph_geometry.rs","glyph_info.rs","glyph_item.rs","glyph_item_iter.rs","glyph_string.rs","item.rs","language.rs","layout.rs","lib.rs","matrix.rs","prelude.rs","rectangle.rs","script_iter.rs","tab_array.rs"]],\
"pango_cairo_sys":["",[],["lib.rs"]],\
"pango_sys":["",[],["lib.rs"]],\
"pangocairo":["",[["auto",[],["font.rs","font_map.rs","functions.rs","mod.rs"]]],["font_map.rs","lib.rs","prelude.rs"]]\
}');
createSourceSidebar();
