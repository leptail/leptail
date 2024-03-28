use leptail::prelude::*;
use leptail_theme_gradiance::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn LoremIpsumLong() -> impl IntoView {
    view! {
        <div class="text-left m-5" >
            <p>
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit,
                sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
                Rhoncus dolor purus non enim praesent elementum facilisis leo vel. 
                Risus at ultrices mi tempus imperdiet. Semper risus in hendrerit gravida rutrum quisque non tellus. 
                Convallis convallis tellus id interdum velit laoreet id donec ultrices. 
                Odio morbi quis commodo odio aenean sed adipiscing. 
                Amet nisl suscipit adipiscing bibendum est ultricies integer quis. 
                Cursus euismod quis viverra nibh cras. 
                Metus vulputate eu scelerisque felis imperdiet proin fermentum leo. 
                Mauris commodo quis imperdiet massa tincidunt. 
                Cras tincidunt lobortis feugiat vivamus at augue. 
                At augue eget arcu dictum varius duis at consectetur lorem. 
                Velit sed ullamcorper morbi tincidunt. Lorem donec massa sapien faucibus et molestie ac.
                Consequat mauris nunc congue nisi vitae suscipit."
            </p>
            <p class="mt-2" >
                "Fringilla est ullamcorper eget nulla facilisi etiam dignissim diam.
                Pulvinar elementum integer enim neque volutpat ac tincidunt. 
                Ornare suspendisse sed nisi lacus sed viverra tellus. 
                Purus sit amet volutpat consequat mauris. Elementum eu facilisis sed odio morbi. 
                Euismod lacinia at quis risus sed vulputate odio. 
                Morbi tincidunt ornare massa eget egestas purus viverra accumsan in. 
                In hendrerit gravida rutrum quisque non tellus orci ac. 
                Pellentesque nec nam aliquam sem et tortor. Habitant morbi tristique senectus et. 
                Adipiscing elit duis tristique sollicitudin nibh sit. 
                Ornare aenean euismod elementum nisi quis eleifend. 
                Commodo viverra maecenas accumsan lacus vel facilisis. 
                Nulla posuere sollicitudin aliquam ultrices sagittis orci a."
            </p>

        </div>
    }
}

#[component]
pub fn NavMenu(
    #[prop(into)] is_open: MaybeSignal<bool>,
) -> impl IntoView {
    let nav_link_class = "px-4 py-2 text-left flex w-full items-start hover:bg-purple-100 hover:dark:bg-purple-900 no-underline hover:no-underline transition-colors duration-100 cursor-pointer";
    let icon_class = "-my-2 -ml-4 w-12 p-3";
    let nav_text_class = move || if is_open() {"flex-1"} else { "hidden"};
    view! {
        <ul class="list-reset">
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class ><Icon icon=icondata::ChMail /></div>
                    <span class=nav_text_class>"Inbox"</span>
                </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class ><Icon icon=icondata::ChStar /></div>
                    <span class=nav_text_class>"Starred"</span>
                </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class ><Icon icon=icondata::BsPencilSquare /></div>
                    <span class=nav_text_class>"Draft"</span> </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class ><Icon icon=icondata::BsSend /></div>
                    <span class=nav_text_class>"Sent"</span> </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class ><Icon icon=icondata::BsTrash /></div>
                    <span class=nav_text_class>"Trash"</span> </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class ><Icon icon=icondata::BsExclamation /></div>
                    <span class=nav_text_class>"Spam"</span> </a>
            </li>
        </ul>
    }
}
