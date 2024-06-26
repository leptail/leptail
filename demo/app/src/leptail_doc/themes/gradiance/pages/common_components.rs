use leptail::prelude::*;
use leptail_theme_gradiance::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn LoremIpsumLong() -> impl IntoView {
    view! {
        <div class="text-left m-5">
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
            <p class="mt-2">
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
pub fn NavMenuExample(#[prop(into)] is_open: MaybeSignal<bool>) -> impl IntoView {
    let nav_link_class = "px-4 py-2 text-left flex w-full items-start hover:bg-purple-100 hover:dark:bg-purple-900 no-underline hover:no-underline transition-colors duration-100 cursor-pointer";
    let icon_class = "-my-2 -ml-4 w-12 p-3";
    let nav_text_class = move || if is_open() { "flex-1" } else { "hidden" };
    view! {
        <ul class="list-reset">
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class>
                        <Icon icon=icondata::ChMail/>
                    </div>
                    <span class=nav_text_class>"Inbox"</span>
                </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class>
                        <Icon icon=icondata::ChStar/>
                    </div>
                    <span class=nav_text_class>"Starred"</span>
                </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class>
                        <Icon icon=icondata::BsPencilSquare/>
                    </div>
                    <span class=nav_text_class>"Draft"</span>
                </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class>
                        <Icon icon=icondata::BsSend/>
                    </div>
                    <span class=nav_text_class>"Sent"</span>
                </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class>
                        <Icon icon=icondata::BsTrash/>
                    </div>
                    <span class=nav_text_class>"Trash"</span>
                </a>
            </li>
            <li>
                <a href="#" class=nav_link_class>
                    <div class=icon_class>
                        <Icon icon=icondata::BsExclamation/>
                    </div>
                    <span class=nav_text_class>"Spam"</span>
                </a>
            </li>
        </ul>
    }
}

#[component]
pub fn OptionalSizeButtons(
    #[prop(into)] size: MaybeSignal<Option<Size>>,
    #[prop(into)] set_size: Out<Option<Size>>,
    #[prop(into)] group_name: &'static str,
) -> impl IntoView {
    let all_sizes = vec![
        Size::XSmall,
        Size::Small,
        Size::Medium,
        Size::Large,
        Size::XLarge,
    ];
    let size_text = |size: &Size| match size {
        Size::XSmall => "Extra Small",
        Size::Small => "Small",
        Size::Medium => "Medium",
        Size::Large => "Large",
        Size::XLarge => "Extra Large",
    };
    let size_buttons = all_sizes
        .into_iter()
        .map(|curr_size| {
            view! {
                <div class="mr-5">
                    <input
                        type="radio"
                        name=group_name
                        checked=move || match size() {
                            Some(s) => s == curr_size,
                            None => false,
                        }

                        on:click=move |_| set_size.set(Some(curr_size))
                    />
                    <span class="ml-2">{size_text(&curr_size)}</span>
                </div>
            }
        })
        .collect_view();

    size_buttons
}

#[component]
pub fn OptionalColorButtons(
    #[prop(into)] color: MaybeSignal<Option<Color>>,
    #[prop(into)] set_color: Out<Option<Color>>,
    #[prop(into)] group_name: &'static str,
) -> impl IntoView {
    let all_colors = vec![
        Color::Default,
        Color::Primary,
        Color::Secondary,
        Color::Info,
        Color::Success,
        Color::Warning,
        Color::Danger,
    ];
    let color_text = |color: &Color| match color {
        Color::Default => "Default",
        Color::Primary => "Primary",
        Color::Secondary => "Secondary",
        Color::Info => "Info",
        Color::Success => "Success",
        Color::Warning => "Warning",
        Color::Danger => "Danger",
    };
    let color_buttons = all_colors
        .into_iter()
        .map(|curr_color| {
            view! {
                <div class="mr-5">
                    <input
                        type="radio"
                        name=group_name
                        checked=move || match color() {
                            Some(s) => s == curr_color,
                            None => false,
                        }

                        on:click=move |_| set_color.set(Some(curr_color))
                    />
                    <span class="ml-2">{color_text(&curr_color)}</span>
                </div>
            }
        })
        .collect_view();

    color_buttons
}
