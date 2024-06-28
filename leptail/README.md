# Component Library for Leptos
Component Library that aims to be headless, themeable using tailwindcss. While being flexible to how developer/designer willing to theme, the library aims to provide off the shelf themes so that one can get started quickly. It also provides(or exposes) `struct` for theme to provide a(some) kind of design system for designer to maintain the consistency across their app.  

## Inputs
- [ ] Autocomplete
- [ ] Button
- [ ] Button Group
- [ ] Checkbox
- [ ] Floating Action Button
- [ ] Radio Group
- [ ] Rating
- [ ] Select
- [ ] Slider
- [x] Switch
- [ ] Text Field
- [ ] Transfer List
- [ ] Toggle Button

## Data display
- [ ] Avatar
- [ ] Badge
- [ ] Chip
- [ ] Divider
- [ ] Icons 
- [ ] List
- [ ] Table
- [ ] Tooltip
- [ ] Typography

## Feedback
- [ ] Alert
- [ ] Backdrop
- [ ] Dialog
- [ ] Progress
- [ ] Skeleton
- [ ] Snackbar

## Surfaces
- [ ] Accordion
- [x] App Bar
- [ ] Card
- [ ] Paper

## Navigation
- [ ] Bottom Navigation
- [ ] Breadcrumbs
- [x] Drawer
- [ ] Link
- [ ] Menu
- [ ] Pagination
- [ ] Speed Dial
- [ ] Stepper
- [ ] Tabs

## Layout
- [ ] Box
- [ ] Container 
- [ ] Grid v2
- [ ] Flex
- [ ] New
- [ ] Stack
- [ ] Image List
- [ ] Hidden

## Utils
- [ ] Click-Away Listener
- [ ] CSS Baseline
- [ ] Modal
- [ ] No SSR
- [ ] Popover
- [ ] Popper
- [ ] Portal
- [ ] Textarea Autosize
- [ ] Transitions
- [ ] useMediaQuery

## MUI X
- [ ] Data Grid
- [ ] Date & Time Pickers

## Lab
- [ ] Masonry
- [ ] Timeline
- [ ] Tree View



## TODO: 
0. Core system:                                                                                                 -- 
    1. Separate the component from the leptail design system.                                                   -- 
    2. Define rules to create leptail component.                                                                -- 
    3. That means anyone can create leptail component, and publish.                                             -- 
    4. AppTheme will be deprectated and every component will get the theme on their own.                        -- done
    5. Provide a helper method to provide the system default theme...                                           -- 
    6. Should there be a marker trait called as leptail theme??? How does it solve any problem?                 -- 
1. Components                                                                                                   --
    1. Core                                                                                                     --
        1. Variant need to be passed as reactive signal                                                         -- done
        2. Keyboard integration and accessability (use leptos_use)                                              --
        3. All the component theme class should have modifiers for each state                                   -- 
    2. Appbar                                                                                                   --
        1. Add footer and aside                                                                                 --
        2. Make screen reader content as component property (optional)                                          -- 
        3. Aria properties for hamburger and close buttons                                                      -- 
        4. What are the aria best practices for appbar??                                                        -- 
        5. Move sr-only class as part of the theme; Currently it's not headless                                 -- 
        6. All the component theme class should have modifiers for each state                                   -- 
    2. Drawer                                                                                                   -- 
        0. Do the code review and optimize                                                                      -- 
        1. Add finegrained animation                                                                            -- 
        2. What are the aria best practices for drawer??                                                        -- 
        3. All the component theme class should have modifiers for each state                                   -- 
    3. Switch Component                                                                                         -- 
        0. Do the code review and optimize                                                                      -- 
        1. Use attr spreding for tabindex, etc                                                                  -- 
        2. How should the underlying input element (i.e form element) should be handled?                        --
        3. What are the aria best practices for switch??                                                        -- 
        4. on_icon and off_icon may not be needed since it is part of the theme, and variant can provide it     -- 
    4. Overlay                                                                                                  -- 
        1. Do the code review and optimize                                                                      -- 
        2. Move it and make it part of the core(primitive) leptail component                                    --
2. Themes                                                                                                       --
    0. Investigate if using https://github.com/Oyelowo/twust is helpful                                         --
    1. Complete the theming for existing components and test it                                                 --
    2. Review and refactor the theme generation code.                                                           --           
    3. Gradiance and Moonlight theme should be equivalent                                                       --
    4. Appbar                                                                                                   --
        1. Provide system default                                                                               --
        2. Setter method for optional values, follow switch theme pattern                                       -- done
        3. Improve the theme                                                                                    -- 
    5. Drawer: improve the theme                                                                                --
        1. Use builder pattern                                                                                  -- done
        2. Improve the theme                                                                                    --
    3. Switch Component                                                                                         --
        1. Use builder pattern                                                                                  -- done 
        2. Improve the theme                                                                                    -- 
3. Build Tool                                                                                                   --
    1. Leptail library and theme installation should be easy to do                                              -- 
    2. There should be a option to merge app's tailwind config with theme's tailwind config                     --  
    3. Reduce the CSS bundle size. It's very high                                                               --
        0. Safelist pattern in tailwind config is increasing the bundle size; Find a ways to avoid it           --  
        1. The moonlight theme takes: 6.5M(uncompressed), 430k(gzip), 160k(br)                                  --
        2. The gradiance theme takes: 172k(uncompressed), 24k(gzip), 11k(br)                                    --
4. Demo/Documentaiton                                                                                           -- 
    0. Show the demo of component for each theme in tab instead of one after the one.                           --
        1. Switch Doc                                                                                           -- 
        2. Drawer Doc                                                                                           -- 
        3. Appbar Doc                                                                                           -- 
        4. Overlay Doc                                                                                          -- 
    1. Allow user to dynamically change the variants in documentation                                           --
    2. Add installation(or getting started) instructions                                                        --
    3. Serve the optimized static files (https://github.com/leptos-rs/cargo-leptos/pull/165)                    --



## Design Decisions
1. Arguments about component variants should be provided by theme! 
    1. Thirdparty theme provider will have more flexibility, thus increasing the innovation 
    2. Variants are not really part of reactivity; thus it's not part of core component lib
    3. Switching theme may need to change code especially if provided by thirdparty
    4. Performance: 
    5. Bundle Size:  
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
    1. There should be basic doumentation for the component usage. 
    2. Theme should provide it's own documentation with variants and variant builder API example. 
    3. Questions:
        1. Will there be change of theme or switch theme in the main documentation section?  




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

