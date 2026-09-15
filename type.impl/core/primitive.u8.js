(function() {
    var type_impls = Object.fromEntries([["i_overlay",[]],["libc",[]],["zmij",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[16,12,12]}