# Component Library for Leptos
Component Library that aims to be headless, themeable using tailwindcss. While being flexible to how developer/designer willing to theme, the library aims to provide off the shelf themes so that one can get started quickly. It also provides(or exposes) `struct` for theme to provide a(some) kind of design system for designer to maintain the consistency across their app.  

## Inputs
    [] Autocomplete
    [] Button
    [] Button Group
    [] Checkbox
    [] Floating Action Button
    [] Radio Group
    [] Rating
    [] Select
    [] Slider
    [X] Switch
    [] Text Field
    [] Transfer List
    [] Toggle Button
## Data display
    [] Avatar
    [] Badge
    [] Chip
    [] Divider
    [] Icons
    [] Material Icons
    [] List
    [] Table
    [] Tooltip
    [] Typography
## Feedback
    [] Alert
    [] Backdrop
    [] Dialog
    [] Progress
    [] Skeleton
    [] Snackbar
## Surfaces
    [] Accordion
    [] App Bar
    [] Card
    [] Paper
## Navigation
    [] Bottom Navigation
    [] Breadcrumbs
    [] Drawer
    [] Link
    [] Menu
    [] Pagination
    [] Speed Dial
    [] Stepper
    [] Tabs
## Layout
    [] Box
    [] Container 
    [] Grid v2
    [] Flex
    [] New
    [] Stack
    [] Image List
    [] Hidden
## Utils
    [] Click-Away Listener
    [] CSS Baseline
    [] Modal
    [] No SSR
    [] Popover
    [] Popper
    [] Portal
    [] Textarea Autosize
    [] Transitions
    [] useMediaQuery
## MUI X
    [] Data Grid
    [] Date & Time Pickers
## Lab
    [] Masonry
    [] Timeline
    [] Tree View

## TODO: 
1. Create a backdrop/overlay component                                                                      -- 
2. Create Drawer component (which uses backdrop) and use it to make appbar                                  -- 
5. Animation: Can you use already existing animations                                                       -- 
    1. Add animiation for drawer                                                                            --  
6. Keyboard integration and accessability                                                                   -- 
    1. Use leptos use whereever possible                                                                    -- 
7. Benchmark and get inspired from the following libraries                                                  --  
    1. https://zagjs.com/components/react/switch                                                            -- 
    2. https://www.radix-ui.com/primitives/docs/components/switch                                           --  
    3. https://ariakit.org/components                                                                       --  
    4. https://kobalte.dev/docs/core/components/switch                                                      --  
8. Leptail library and theme installation should be easy to do                                              -- 
    1. There should be a option to merge app's tailwind config with theme's tailwind config                 --  
9. Investigate if using https://github.com/Oyelowo/twust is helpful                                         --  
10. Drawer implemenation                                                                                    -- 
    1. should take the minDim and maxDim                                                                    --
    2. What is the data type of dim? How should it get updated?                                             -- 
    3. Is it possible to use tailwindcss animations instead of using RAF??                                  -- done
        Ans: Yes
    4. Remove the close button from drawer. Let the user decide if there need to be close button            -- done
    5. The app-bar can have drawer that can have intermidate (for like icon drawer, full menu drawer)       -- 
        1. The leptail app can have two appbars                                                             -- 
            1. One used by home page                                                                        -- 
                1. Drawer variant: more web landing page looking app;                                       -- 
                2. Layout brakeover: 2xl or xl customizazble; 
            2. One used by doc pages                                                                        --
                1. Drawer variant: persistent drawer on desktop;                                            -- 
                2. Layout brakeover: full width page                                                        --
11. Check if we can use tailwind fuse library                                                               -- 
    1. https://github.com/gaucho-labs/tailwind-fuse
11. Take a look at Taildwind Variants and NextUI                                                            -- 
    0. It may help in designing themes...                                                                   -- 
    1. https://github.com/nextui-org/tailwind-variants

## Design Decisions
1. Arguments about component variants should be provided by theme! 
    1. For: 
        1. Thirdparty theme provider will have more flexibility, thus increasing the innovation 
        2. Variants are not really part of reactivity; thus it's not part of core component lib
        3. Switching theme may need to change code especially if provided by thirdparty
        4. Performance: 
        5. Bundle Size: 
    2. Against: 
        1. Being part of core libarary will improve consistency; 
        2. switching theme will not change change force users to change the code 
        3. Performance: 
        4. Bundle Size: 
2. Component Ergonomics 
    1. How should state be passed? 
        -- Follow how leptonic is doing
    2. How should the event be handled 
        -- For state change event follow how leptonic is doing. For other types of event think about it
    3. How should ARIA work?
        -- Best is wrap ARIA related configuration in a option of struct and pass it 
    4. How should keyboard events should be handled? 
        -- todo:
    5. How should focus be handled?
        -- todo:
    6. How should onLoad events be registered?
        -- todo: 
    7. Who should provide the Icons? 
        -- Use a fallback icons from the leptos icons. 
        -- Icons should be provided by theme; but optional 
        -- Icons can also be provided by the library users. 
3. How should the Documentaion work? Especially with themes? 
    1. There should be basic doumentation for the component usage. That doesn't specifies the theme variants but only the API 
    2. Theme should provide it's own documentation with variants etc. 
    3. Questions:
        1. Will there be change of theme or switch theme in the main documentation section?  


## Performance and Bundle Size: 
1. Can tree shaking be possible? or is there a way to selectively choose the components? 
2. CSS bundle size of 2MB for gradiance theme. How can it be optimized? Does leptos allow css to be gziped?




## Design best practices
When developing headless UI components, which are UI components that provide functionality without dictating the visual presentation, there are several key considerations to keep in mind to ensure their effectiveness and reusability:

**Accessibility:** Accessibility should be a top priority. Ensure that the component is fully accessible to users with disabilities. Use semantic HTML elements and provide appropriate ARIA attributes and roles. Test with screen readers and keyboard navigation to ensure a seamless experience.

**Customization:** Headless UI components should be highly customizable. Allow developers to easily apply their own styles, including CSS classes and styles. Avoid hardcoding styles or assumptions about the component's appearance.

**Separation of Concerns:** Separate the logic and behavior of the component from its presentation. This enables developers to integrate the component into different design systems and adapt it to various visual styles.

**API Design:** Design a clean and intuitive API for the component. Consider the needs of both beginners and experienced developers. Provide clear documentation and examples to guide users in implementing and customizing the component.

**Event Handling:** Ensure that the component handles user interactions gracefully. Allow developers to attach event listeners to the component and provide callbacks for common events such as clicks, focus, or input changes.

**Testability:** Make it easy to test the component's behavior in isolation. Encourage unit testing and provide testing utilities or guidance on how to test the component effectively.

**Dependencies:** Minimize dependencies on external libraries or frameworks. If dependencies are necessary, clearly document them and their versions. Avoid tightly coupling the component to a specific technology stack.

**Performance:** Optimize the component for performance. Avoid unnecessary re-renders and ensure that it works efficiently in different contexts, including server-side rendering (SSR) and single-page applications (SPAs).

**Browser Compatibility:** Test the component in various web browsers to ensure broad compatibility. Consider polyfills or fallbacks for features that may not be supported in older browsers.

**State Management:** Decide whether the component should manage its own state or rely on external state management solutions. Provide flexibility for developers to choose their preferred state management approach.

**Internationalization (i18n) and Localization (l10n):** Allow for easy integration with internationalization and localization libraries or practices. Ensure that text and messages are easily translatable.

**Error Handling:** Provide clear error messages and handling mechanisms for common issues developers might encounter when using the component.

**Security:** Consider security best practices, especially if the component handles user input or data. Guard against potential vulnerabilities like cross-site scripting (XSS) and data injection.

**Browser Features:** Be aware of browser features and limitations, such as support for CSS custom properties (variables) and the Shadow DOM, which can impact how the component is styled and used.

**Community and Feedback:** Encourage feedback and contributions from the developer community. An active community can help identify and address issues, improve documentation, and extend the component's capabilities.

**Versioning and Compatibility:** Implement a versioning strategy for the component to manage changes and updates. Clearly communicate breaking changes and provide migration guides when necessary.

**Documentation and Examples:** Comprehensive documentation with clear usage examples is crucial for adoption. Include code samples, best practices, and troubleshooting guidance.

**Browser DevTools Support:** If applicable, ensure that the component works well with browser development tools, making it easier for developers to inspect and debug.

**Performance Profiling:** Provide guidance on how to profile and optimize the component's performance, including strategies for lazy loading and code splitting.

