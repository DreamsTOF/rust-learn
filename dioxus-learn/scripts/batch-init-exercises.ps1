$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$helper = Join-Path $scriptPath "new-exercise.ps1"
if (-not (Test-Path $helper)) { Write-Error "Missing $helper"; exit 1 }
$ok = 0; $err = @()
$exercises = @(
@("00_setup",1,"install_rust"),@("00_setup",2,"install_cli"),@("00_setup",3,"create_project"),@("00_setup",4,"run_dev_server"),@("00_setup",5,"editor_config"),
@("00_setup",6,"rust_toolchain"),@("00_setup",7,"first_component"),@("00_setup",8,"build_release"),@("00_setup",9,"browser_devtools"),@("00_setup",10,"project_template"),
@("01_basics",11,"hello_world"),@("01_basics",12,"text_nodes"),@("01_basics",13,"html_elements_attributes"),@("01_basics",14,"element_nesting"),@("01_basics",15,"component_definition"),
@("01_basics",16,"component_nesting"),@("01_basics",17,"fragment_syntax"),@("01_basics",18,"comment_syntax"),@("01_basics",19,"rust_expression"),@("01_basics",20,"block_expression"),
@("01_basics",21,"conditional_if"),@("01_basics",22,"match_expression"),@("01_basics",23,"list_iterator"),@("01_basics",24,"list_for_loop"),@("01_basics",25,"key_attribute"),
@("01_basics",26,"svg_elements"),@("01_basics",27,"dangerous_inner_html"),@("01_basics",28,"style_attributes"),@("01_basics",29,"dynamic_tag_name"),@("01_basics",30,"attribute_spread"),
@("02_signals",31,"use_signal_create"),@("02_signals",32,"read_reference"),@("02_signals",33,"function_call_read"),@("02_signals",34,"display_read"),@("02_signals",35,"set_value"),
@("02_signals",36,"write_mutable"),@("02_signals",37,"operator_overload"),@("02_signals",38,"toggle_bool"),@("02_signals",39,"iter_collection"),@("02_signals",40,"multiple_signals"),
@("02_signals",41,"type_inference"),@("02_signals",42,"read_signal_vs_signal"),@("02_signals",43,"lazy_init"),@("02_signals",44,"signal_copy"),@("02_signals",45,"peek_non_reactive"),
@("02_signals",46,"signal_async_boundary"),@("02_signals",47,"auto_batching"),
@("02_signals",48,"simple_move_closure"),@("02_signals",49,"multi_signal_derived"),@("02_signals",50,"use_memo_basic"),@("02_signals",51,"memo_vs_closure"),@("02_signals",52,"memo_chained"),
@("02_signals",53,"memo_conditional"),@("02_signals",54,"lazy_derived"),@("02_signals",55,"derived_with_function"),@("02_signals",56,"signal_array_derived"),@("02_signals",57,"reactive_eq"),
@("02_signals",58,"derived_signal_prop"),@("02_signals",59,"memo_read_method"),@("02_signals",60,"memo_cloned"),@("02_signals",61,"conditional_propagation"),@("02_signals",62,"dependency_tracking"),
@("02_signals",63,"memo_signal_interop"),
@("02_signals",64,"use_effect_basic"),@("02_signals",65,"effect_react_signal"),@("02_signals",66,"effect_dependency_tracking"),@("02_signals",67,"effect_no_external_tracking"),@("02_signals",68,"multiple_effects"),
@("02_signals",69,"effect_conditional"),@("02_signals",70,"use_drop_cleanup"),@("02_signals",71,"effect_async"),@("02_signals",72,"avoid_infinite_loop"),@("02_signals",73,"batched_update"),
@("02_signals",74,"untrack"),@("02_signals",75,"effect_debug_logger"),@("02_signals",76,"use_hook_primitive"),@("02_signals",77,"use_signal_vs_use_hook"),@("02_signals",78,"component_rerender_condition"),
@("02_signals",79,"zero_cost_reactivity"),@("02_signals",80,"needs_update"),@("02_signals",81,"effect_execution_timing"),@("02_signals",82,"effect_read_memo"),@("02_signals",83,"effect_access_dom"),
@("02_signals",84,"conditional_effect"),
@("02_signals",85,"global_signal_create"),@("02_signals",86,"global_signal_read_write"),@("02_signals",87,"global_signal_shared"),@("02_signals",88,"global_local_mixed"),@("02_signals",89,"global_signal_type_constraint"),
@("02_signals",90,"global_signal_ssr_isolation"),
@("02_signals",91,"use_ref_basic"),@("02_signals",92,"use_ref_vs_signal"),@("02_signals",93,"multi_signal_sync"),@("02_signals",94,"signal_as_prop"),@("02_signals",95,"read_signal_prop"),
@("02_signals",96,"vec_push"),@("02_signals",97,"vec_remove"),@("02_signals",98,"hashmap_signal"),@("02_signals",99,"signal_split"),@("02_signals",100,"signal_map"),@("02_signals",101,"signal_filter"),
@("02_signals",102,"signal_async_interop"),@("02_signals",103,"signal_closure_capture"),@("02_signals",104,"lazy_evaluation"),@("02_signals",105,"reactivity_performance_model"),
@("03_components",106,"function_component"),@("03_components",107,"component_macro"),@("03_components",108,"inline_props"),@("03_components",109,"multiple_props"),@("03_components",110,"optional_props"),
@("03_components",111,"props_default"),@("03_components",112,"props_into"),@("03_components",113,"struct_props"),@("03_components",114,"partial_eq_custom"),@("03_components",115,"clone_requirement"),
@("03_components",116,"props_spread"),@("03_components",117,"doc_comments"),@("03_components",118,"component_vs_function"),@("03_components",119,"component_lifecycle"),@("03_components",120,"pure_function_vs_component"),
@("03_components",121,"children_element"),@("03_components",122,"pass_children"),@("03_components",123,"nested_children"),@("03_components",124,"wrapper_component"),@("03_components",125,"multiple_slots"),
@("03_components",126,"children_type_constraint"),@("03_components",127,"layout_component"),@("03_components",128,"fragment_implementation"),@("03_components",129,"conditional_children"),@("03_components",130,"iterate_children"),
@("03_components",131,"dynamic_children"),@("03_components",132,"context_provider_component"),@("03_components",133,"higher_order_component"),@("03_components",134,"render_prop"),@("03_components",135,"spread_override"),
@("03_components",136,"local_state_in_component"),@("03_components",137,"callback_prop"),@("03_components",138,"event_handler_type"),@("03_components",139,"two_way_binding"),@("03_components",140,"controlled_vs_uncontrolled"),
@("03_components",141,"pass_signal_prop"),@("03_components",142,"pass_closure_prop"),@("03_components",143,"generic_component"),@("03_components",144,"reusable_design"),@("03_components",145,"rerender_optimization"),
@("03_components",146,"component_cache"),@("03_components",147,"context_in_component"),@("03_components",148,"multiple_hooks"),@("03_components",149,"async_init"),@("03_components",150,"conditional_mount"),
@("03_components",151,"key_retain_state"),@("03_components",152,"composition_vs_inheritance"),@("03_components",153,"atomic_components"),@("03_components",154,"compound_component"),@("03_components",155,"style_encapsulation"),
@("04_events_forms",156,"onclick_event"),@("04_events_forms",157,"oninput_event"),@("04_events_forms",158,"onchange_event"),@("04_events_forms",159,"onsubmit_event"),@("04_events_forms",160,"event_object"),
@("04_events_forms",161,"keyboard_events"),@("04_events_forms",162,"mouse_events"),@("04_events_forms",163,"focus_events"),@("04_events_forms",164,"scroll_events"),@("04_events_forms",165,"onresize_event"),
@("04_events_forms",166,"onvisible_event"),@("04_events_forms",167,"event_bubbling"),@("04_events_forms",168,"prevent_default"),@("04_events_forms",169,"handler_return_error"),@("04_events_forms",170,"custom_events"),
@("04_events_forms",171,"text_input_binding"),@("04_events_forms",172,"number_input"),@("04_events_forms",173,"checkbox_input"),@("04_events_forms",174,"radio_input"),@("04_events_forms",175,"select_input"),
@("04_events_forms",176,"textarea_input"),@("04_events_forms",177,"file_upload"),@("04_events_forms",178,"validation_required"),@("04_events_forms",179,"validation_email"),@("04_events_forms",180,"validation_password"),
@("04_events_forms",181,"form_submit"),@("04_events_forms",182,"form_reset"),@("04_events_forms",183,"form_loading"),@("04_events_forms",184,"multi_step_form"),@("04_events_forms",185,"custom_form_control"),
@("04_events_forms",186,"conditional_attribute"),@("04_events_forms",187,"multiple_class"),@("04_events_forms",188,"inline_style"),@("04_events_forms",189,"drag_events"),@("04_events_forms",190,"clipboard_events"),
@("04_events_forms",191,"touch_events"),@("04_events_forms",192,"animation_events"),@("04_events_forms",193,"dangerous_inner_html_advanced"),@("04_events_forms",194,"custom_data_attributes"),@("04_events_forms",195,"inline_javascript"),
@("05_async",196,"use_resource_basic"),@("05_async",197,"resource_read"),@("05_async",198,"resource_reactive_dep"),@("05_async",199,"resource_loading_state"),@("05_async",200,"resource_suspend"),
@("05_async",201,"suspense_boundary_basic"),@("05_async",202,"suspense_multi_resource"),@("05_async",203,"suspense_nested"),@("05_async",204,"resource_refresh"),@("05_async",205,"resource_dependency_chain"),
@("05_async",206,"resource_error_handling"),@("05_async",207,"resource_timeout"),@("05_async",208,"resource_polling"),@("05_async",209,"resource_cancel"),@("05_async",210,"optimistic_ui"),
@("05_async",211,"spawn_basic"),@("05_async",212,"spawn_update_signal"),@("05_async",213,"spawn_local_vs_spawn"),@("05_async",214,"async_event_handler"),@("05_async",215,"join_concurrent"),
@("05_async",216,"select_race"),@("05_async",217,"use_callback"),@("05_async",218,"signal_async_capture"),@("05_async",219,"async_borrow_temporary"),@("05_async",220,"lazy_load"),
@("05_async",221,"gloo_net_http"),@("05_async",222,"reqwest_wasm"),@("05_async",223,"json_serialize"),@("05_async",224,"websocket_basic"),@("05_async",225,"use_websocket_fullstack"),
@("05_async",226,"render_error"),@("05_async",227,"question_mark_operator"),@("05_async",228,"error_boundary_basic"),@("05_async",229,"error_boundary_nested"),@("05_async",230,"event_handler_error"),
@("05_async",231,"anyhow_integration"),@("05_async",232,"error_downcast"),@("05_async",233,"local_error_state"),@("05_async",234,"error_recovery_retry"),@("05_async",235,"global_error_strategy"),
@("06_router",236,"install_router"),@("06_router",237,"routable_enum"),@("06_router",238,"render_router"),@("06_router",239,"link_navigation"),@("06_router",240,"path_param"),
@("06_router",241,"query_param"),@("06_router",242,"wildcard_route"),@("06_router",243,"nested_routes"),@("06_router",244,"layout_route"),@("06_router",245,"redirect"),
@("06_router",246,"navigator_push"),@("06_router",247,"navigator_replace"),@("06_router",248,"navigator_go_back"),@("06_router",249,"not_found_route"),@("06_router",250,"active_class"),
@("06_router",251,"route_guard"),@("06_router",252,"lazy_loading_route"),@("06_router",253,"nested_layout"),@("06_router",254,"multi_param_route"),@("06_router",255,"optional_param"),
@("06_router",256,"query_param_serialize"),@("06_router",257,"ssr_route"),@("06_router",258,"use_route_current"),@("06_router",259,"cross_route_state"),@("06_router",260,"navigate_with_data"),
@("06_router",261,"route_transition_animation"),@("06_router",262,"tab_navigation"),@("06_router",263,"breadcrumb_navigation"),@("06_router",264,"before_leave_confirm"),@("06_router",265,"no_router_spa"),
@("06_router",266,"use_routing_state"),@("06_router",267,"link_custom_style"),@("06_router",268,"link_target"),@("06_router",269,"dynamic_route_generation"),@("06_router",270,"route_param_validation"),
@("06_router",271,"route_use_resource"),@("06_router",272,"lazy_loading_suspense"),@("06_router",273,"reactive_navigation"),@("06_router",274,"external_link"),@("06_router",275,"route_change_listener"),
@("06_router",276,"multi_level_layout"),@("06_router",277,"auth_integration"),@("06_router",278,"route_param_resource"),@("06_router",279,"ssr_route_matching"),@("06_router",280,"full_multi_page_app"),
@("07_context_state",281,"use_context_provider"),@("07_context_state",282,"use_context_consumer"),@("07_context_state",283,"context_type_safety"),@("07_context_state",284,"context_deep_penetration"),@("07_context_state",285,"multiple_context"),
@("07_context_state",286,"context_provider_component"),@("07_context_state",287,"dynamic_provide_context"),@("07_context_state",288,"dynamic_consume_context"),@("07_context_state",289,"scoped_context"),@("07_context_state",290,"context_with_signals"),
@("07_context_state",291,"context_methods"),@("07_context_state",292,"context_async_methods"),@("07_context_state",293,"cross_route_context"),@("07_context_state",294,"context_store_element"),@("07_context_state",295,"large_app_state_architecture"),
@("07_context_state",296,"global_signal_declare"),@("07_context_state",297,"global_signal_safety"),@("07_context_state",298,"global_signal_cross_component"),@("07_context_state",299,"multiple_global_signals"),@("07_context_state",300,"global_local_mixed_usage"),
@("07_context_state",301,"global_signal_init_timing"),@("07_context_state",302,"global_signal_ssr_isolation"),@("07_context_state",303,"global_signal_map"),@("07_context_state",304,"global_signal_filter"),@("07_context_state",305,"global_vs_local_choice"),
@("07_context_state",306,"derive_store_basic"),@("07_context_state",307,"store_lens_access"),@("07_context_state",308,"store_nested"),@("07_context_state",309,"store_write_update"),@("07_context_state",310,"store_option_result"),
@("07_context_state",311,"store_hashmap"),@("07_context_state",312,"store_vec"),@("07_context_state",313,"store_lens_props"),@("07_context_state",314,"read_store"),@("07_context_state",315,"store_performance"),
@("08_fullstack",316,"fullstack_project_create"),@("08_fullstack",317,"fullstack_cargo_config"),@("08_fullstack",318,"ssr_rendering_basic"),@("08_fullstack",319,"hydration"),@("08_fullstack",320,"hot_reload_fullstack"),
@("08_fullstack",321,"release_build"),@("08_fullstack",322,"fullstack_project_structure"),@("08_fullstack",323,"environment_config"),@("08_fullstack",324,"cors_config"),@("08_fullstack",325,"deploy_fly_io"),
@("08_fullstack",326,"server_fn_get"),@("08_fullstack",327,"server_fn_post"),@("08_fullstack",328,"server_fn_put"),@("08_fullstack",329,"server_fn_delete"),@("08_fullstack",330,"server_fn_path_param"),
@("08_fullstack",331,"server_fn_query_param"),@("08_fullstack",332,"server_fn_client_call"),@("08_fullstack",333,"server_fn_error"),@("08_fullstack",334,"server_fn_database"),@("08_fullstack",335,"server_fn_file_upload"),
@("08_fullstack",336,"server_fn_streaming"),@("08_fullstack",337,"server_fn_websocket"),@("08_fullstack",338,"server_fn_auth"),@("08_fullstack",339,"server_fn_custom_types"),@("08_fullstack",340,"server_fn_batch"),
@("08_fullstack",341,"ssr_workflow"),@("08_fullstack",342,"ssr_data_fetching"),@("08_fullstack",343,"server_future_vs_resource"),@("08_fullstack",344,"streaming_ssr"),@("08_fullstack",345,"ssr_suspense"),
@("08_fullstack",346,"seo_optimization"),@("08_fullstack",347,"static_site_generation"),@("08_fullstack",348,"incremental_static_regeneration"),@("08_fullstack",349,"server_context"),@("08_fullstack",350,"hydration_error_debug"),
@("08_fullstack",351,"sqlite_integration"),@("08_fullstack",352,"crud_operations"),@("08_fullstack",353,"database_migrations"),@("08_fullstack",354,"postgresql_integration"),@("08_fullstack",355,"connection_pool"),
@("09_advanced",356,"custom_hook_basic"),@("09_advanced",357,"hook_composition"),@("09_advanced",358,"hook_use_hook_primitive"),@("09_advanced",359,"hook_use_drop"),@("09_advanced",360,"hook_with_params"),
@("09_advanced",361,"async_custom_hook"),@("09_advanced",362,"hook_context"),@("09_advanced",363,"hook_needs_update"),@("09_advanced",364,"hook_testing"),@("09_advanced",365,"hook_publish_crates_io"),
@("09_advanced",366,"use_persistent"),@("09_advanced",367,"hook_naming_convention"),
@("09_advanced",368,"component_mount_unmount"),@("09_advanced",369,"rerender_triggers"),@("09_advanced",370,"props_partial_eq"),@("09_advanced",371,"no_mutate_in_body"),@("09_advanced",372,"use_memo_derived"),
@("09_advanced",373,"conditional_render_optimization"),@("09_advanced",374,"virtual_node"),@("09_advanced",375,"unsubscribe_on_drop"),
@("09_advanced",376,"web_sys_dom"),@("09_advanced",377,"web_sys_browser_api"),@("09_advanced",378,"js_interpolation"),@("09_advanced",379,"custom_render_attributes"),@("09_advanced",380,"custom_elements"),
@("09_advanced",381,"iframe_integration"),@("09_advanced",382,"eval_provider"),@("09_advanced",383,"third_party_js"),@("09_advanced",384,"performance_profiling"),@("09_advanced",385,"custom_renderer"),
@("09_advanced",386,"dioxus_logger"),@("09_advanced",387,"devtools_integration"),@("09_advanced",388,"log_filter"),@("09_advanced",389,"signal_tracing"),@("09_advanced",390,"error_reporting"),
@("09_advanced",391,"unit_test_component"),@("09_advanced",392,"event_interaction_test"),@("09_advanced",393,"async_component_test"),@("09_advanced",394,"snapshot_test"),@("09_advanced",395,"e2e_test"),
@("10_verification",396,"logger_init"),@("10_verification",397,"log_level_dynamic"),@("10_verification",398,"devtools_component_tree"),@("10_verification",399,"devtools_signal_monitor"),@("10_verification",400,"performance_profiling_basic"),
@("10_verification",401,"rerender_analysis"),@("10_verification",402,"wasm_size_optimization"),@("10_verification",403,"code_splitting"),@("10_verification",404,"memory_leak_detection"),@("10_verification",405,"a11y_audit"),
@("10_verification",406,"seo_check"),@("10_verification",407,"lighthouse_audit"),@("10_verification",408,"unit_test_pure"),@("10_verification",409,"component_render_test"),@("10_verification",410,"event_handler_test"),
@("10_verification",411,"async_component_test"),@("10_verification",412,"ci_cd_integration"),@("10_verification",413,"production_build_verify"),@("10_verification",414,"error_monitoring"),@("10_verification",415,"comprehensive_checklist")
)
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Dioxus Learn - Batch Init Exercises"    -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Total exercises: $($exercises.Count)" -ForegroundColor Yellow
Write-Host "Expected folders (with answers): $($exercises.Count * 2)" -ForegroundColor Yellow
Write-Host ""

$created = 0; $errors = @()
foreach ($ex in $exercises) {
    $chapter = $ex[0]; $number = $ex[1]; $name = $ex[2]
    $r1 = & $helper -Chapter $chapter -Number $number -Name $name -Type "exercise" 2>&1
    if ($LASTEXITCODE -eq 0) { $created++ } else { $errors += "EX FAIL: $chapter $number $name" }
    $r2 = & $helper -Chapter $chapter -Number $number -Name $name -Type "answer" 2>&1
    if ($LASTEXITCODE -eq 0) { $created++ } else { $errors += "AN FAIL: $chapter $number $name" }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Summary"                               -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Folders created: $created / $($exercises.Count * 2)" -ForegroundColor Green
if ($errors.Count -gt 0) {
    Write-Host "Errors: $($errors.Count)" -ForegroundColor Red
    foreach ($e in $errors) { Write-Host "  $e" -ForegroundColor Red }
} else {
    Write-Host "All folders created successfully!" -ForegroundColor Green
}
