# Component Library for Leptos
Headless Component Library with Default Design System.  

## Complete refactoring of the project 
When the project was conceived, I had a small expectation from it. All I wanted was a component library that works with SSR(at that time there was none) and be kind of headless(so that other users besides me can use it with different themes/designs). My preferred way of styling was through tailwindcss! That is what I wanted to support. Little did I know at that time what kind of rabbit hole I was getting into. I wrote a basic component switch(since I was also new to the rust ecosystem altogether) to start experimenting. While writing the switch component, I started to search regarding best practices for that component. Then bumped on to WAI-ARIA guidelines, then on to react-aria implementation, then to radixUI etc, etc. However, I went ahead with development and created a couple of more(drawer and appbar) components. Fiddled around with themming to create component variants etc. After finishing(more are less) the work with variant ergonomics etc, there was a moment of reflection with what has been done so far. So I went into the rabbit hole, and started questioning all the design decisions, etc even to the simple question such as why did I even choose tailwind to provide themes? Out of the reflection, I came out with the following thought.

1. Accessibility is hard! It needs a lot of testing.
2. Design system is another beast!

Also, while reading on the web, I came across stories about the time spent by [Adobe team on combobox](https://react-spectrum.adobe.com/blog/building-a-combobox.html), and [radix team on dropdown implementation](https://www.youtube.com/watch?v=lY-RQjWeweo&t=1395s). With the limited time, it is not worth re-inventing the entire wheel but rather build on top of what people have done already. With that in mind, the next refactor or ground-up implementation will have the following structure.

1. Core API, (similar to aria-hooks, react stately, zag)
2. Primitive Component (similar to aria components or radix primitives, floating-ui)
3. Design System and Components ([radix themes](https://github.com/radix-ui/themes), et al)
4. Things which are neither part of components or design-system(built using above three), provide it as part of building block(or examples) and let people copy paste(like ShadcnUI)

With the above structure, some of the things are fixed as broad guidelines.
1. Follow [react-aria](https://github.com/adobe/react-spectrum) for core API (mostly because I believe they have tested things extensively)
2. [Open props](https://open-props.style/) looks promising for a design system. That coupled with Radix theme, chakra or MUI base can help to design the API.
3. Will probably need some tooling like compile time css in js such as [linaria](https://github.com/callstack/linaria), [compliedcssinjs](https://compiledcssinjs.com/), [vanilla extract](https://github.com/vanilla-extract-css/vanilla-extract), [sitches](https://stitches.dev/), [rainbow sprinkles](https://github.com/wayfair/rainbow-sprinkles), [stylex by facebook](https://github.com/facebook/stylex), and [comparision of many such libs](https://github.com/andreipfeiffer/css-in-js?tab=readme-ov-file#motivation).
4. Follow Rust's philosophy. Only pay for what you use! Means no unused css or code. Use rust lang features. 
5. Make it easier to build the component outside of leptail (using core apis) and publish. Others can still import and use it.
6. Afterall, it should also work with tailwindcss :). It is important for library adoption. But don't sweat it with the default theme or full blown design system using tailwindcss. 

*Needs further investigation:*
1. API design of [headlessui](https://github.com/tailwindlabs/headlessui), [Floating UI](https://github.com/floating-ui/floating-ui), [radix primitives](https://github.com/radix-ui/primitives), [BaseUI](https://github.com/mui/base-ui), [ArkUI](https://github.com/chakra-ui/ark)
2. [Zag](https://github.com/chakra-ui/zag) vs aria hooks/stately.
3. How should the animation work? Open props does provide nice animations. But do we need more than that? How does floatingUI handle it?
4. How does LTR and RTL work?
5. Can it be composable? How easy will it be??
6. The difference between react and leptos as framework that enforce constraint on component API design
7. What about internationalisation? How do they work in terms of accessibility?
8. What about people who wants to create a [facade over the library](https://old.reddit.com/r/reactjs/comments/1ai7yi2/good_examples_of_component_libraries_built_on_top/kotbpmu/)? How will that work?
9. How much of the HTML structure will be fixed? Headless v/s unstyled! 


*Nice goal to have:*
1. Whatever you do, make it performant! Not a bloated thing. Even the big app built with it should score 100% on lighthouse(ha ha... lol)

*References:*
1. [Use of attributes for styling](https://pdx.su/blog/2023-07-27-use-css-attributes/)
2. The [article comparing aria with radix](https://www.dhiwise.com/post/react-aria-vs-radix-ui-what-best-ui-toolkit) lists the following features as common among them
   1. Accessibility
   2. Headless/Unstyled
   3. Behaviour Hooks
   4. Composability
   5. Focus Management
   6. State Management
3. There is a real world comparison between [styled component vs css modules](https://pustelto.com/blog/css-vs-css-in-js-perf/)
4. [Atomic/Functional CSS v/ CSS Module](https://www.fcss.club/manifesto) comparision of performance. 


### TODO:
1. Restructure the code (Remove old code)                                           -- 
   1. Delete everything and create the new structure                                -- done
   2. Upgrade to leptos 0.7                                                         -- done
   3. Try out open props and stylance ind demo                                      -- done
2. Implement Switch component                                                       --
   1. Necessary core API                                                            -- 
   2. Switch as component                                                           -- 
      1. API design                                                                 -- 
         1. As part of form component                                               --
         2. As standalone component                                                 --  
      2. A18y                                                                       --
      3. Internationalization                                                       --
      4. LTR / RTL                                                                  --
   3. Styled component                                                              --
      1. Install [stylance](https://github.com/basro/stylance-rs)                   --
      2. Use OpenProps                                                              --
   4. Example or documentation                                                      --
3. Testing outside of letptail                                                      --


## Best practices
Copied from somewhere on the web. 

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


# List of component the MUI has
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
- [ ] Switch
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
- [ ] App Bar
- [ ] Card
- [ ] Paper

## Navigation
- [ ] Bottom Navigation
- [ ] Breadcrumbs
- [ ] Drawer
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