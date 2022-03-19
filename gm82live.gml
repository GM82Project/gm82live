#define __gm82live_init
    globalvar __gm82live_listen;__gm82live_listen=noone


//live room editor module

#define __gm82live_re_init
    with (__gm82core_object) {
        if (__gm82live_listen==noone) {
            object_event_add(__gm82core_object,ev_step,ev_step_begin,"__gm82live_re_poll()")
            __gm82live_listen=listeningsocket_create()
            listeningsocket_start_listening(__gm82live_listen,0,4126,1)
            __gm82live_sock=noone
        }
    }


#define __gm82live_re_poll
    if (listeningsocket_can_accept(__gm82live_listen)) {
        __gm82live_sock=socket_create()
        __gm82live_buf=buffer_create()
        listeningsocket_accept(__gm82live_listen,__gm82live_sock)
    }
    var __obj,__i;
    if (__gm82live_sock!=noone) {
        socket_update_read(__gm82live_sock)
        while (socket_read_message(__gm82live_sock,__gm82live_buf)) {
            buffer_set_pos(__gm82live_buf,0)
            repeat (buffer_read_u32(__gm82live_buf)) {
                __obj=buffer_read_u32(__gm82live_buf)
                with (__obj) if (object_index==__obj) instance_destroy()
                repeat (buffer_read_u32(__gm82live_buf)) {
                    __i=instance_create(buffer_read_i32(__gm82live_buf),buffer_read_i32(__gm82live_buf),__obj)
                    __i.image_xscale=buffer_read_double(__gm82live_buf)
                    __i.image_yscale=buffer_read_double(__gm82live_buf)
                    __i.image_angle=buffer_read_double(__gm82live_buf)
                    __i.image_blend=buffer_read_i32(__gm82live_buf)
                    __i.image_alpha=buffer_read_double(__gm82live_buf)
                    with (__i) execute_string(buffer_read_string(other.__gm82live_buf))
                }
            }
        }
    }


//live code editor module

#define __gm82live_fw_init
    __gm82live_dll_fw_init(working_directory+"\")


#define __gm82live_fw_poll
    return __gm82live_dll_fw_poll()
//
//