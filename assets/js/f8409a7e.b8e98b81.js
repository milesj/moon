"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[3206],{4e3:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>a,contentTitle:()=>c,default:()=>g,frontMatter:()=>o,metadata:()=>d,toc:()=>h});var s=n(24246),i=n(71670),r=n(59220),l=n(9619);const o={slug:"/",title:"Introduction"},c=void 0,d={id:"intro",title:"Introduction",description:"moonrepo is a productivity platform that aims to eliminate pain points for both developers and",source:"@site/docs/intro.mdx",sourceDirName:".",slug:"/",permalink:"/docs/",draft:!1,unlisted:!1,editUrl:"https://github.com/moonrepo/moon/tree/master/website/docs/intro.mdx",tags:[],version:"current",frontMatter:{slug:"/",title:"Introduction"},sidebar:"docs",next:{title:"Install moon",permalink:"/docs/install"}},a={},h=[{value:"moon",id:"moon",level:2},{value:"Why use moon?",id:"why-use-moon",level:3},{value:"Supported languages",id:"supported-languages",level:3},{value:"Supported targets",id:"supported-targets",level:3},{value:"Features",id:"features",level:3},{value:"Management",id:"management",level:4},{value:"Organization",id:"organization",level:4},{value:"Orchestration",id:"orchestration",level:4},{value:"Notification",id:"notification",level:4},{value:"moonbase",id:"moonbase",level:2},{value:"proto",id:"proto",level:2}];function x(e){const t={a:"a",code:"code",em:"em",h2:"h2",h3:"h3",h4:"h4",li:"li",p:"p",strong:"strong",table:"table",tbody:"tbody",td:"td",th:"th",thead:"thead",tr:"tr",ul:"ul",...(0,i.a)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(t.p,{children:"moonrepo is a productivity platform that aims to eliminate pain points for both developers and\ncompanies, by automating tiresome and complex workflows, and improving the overall developer\nexperience."}),"\n",(0,s.jsx)(t.p,{children:"We currently achieve this through the following tools and services:"}),"\n",(0,s.jsx)(t.h2,{id:"moon",children:"moon"}),"\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.a,{href:"/moon",children:"moon"})," is a repository ",(0,s.jsx)(t.em,{children:"m"}),"anagement, ",(0,s.jsx)(t.em,{children:"o"}),"rganization, ",(0,s.jsx)(t.em,{children:"o"}),"rchestration, and ",(0,s.jsx)(t.em,{children:"n"}),"otification tool\nfor the web ecosystem, written in Rust. Many of the concepts within moon are heavily inspired from\nBazel and other popular build systems, but tailored for our\n",(0,s.jsx)(t.a,{href:"#supported-languages",children:"supported languages"}),"."]}),"\n",(0,s.jsx)(t.p,{children:"You can think of a moon as a tool that sits firmly in the middle between Bazel (high complexity,\nfull structure), and make/just/etc scripts (low complexity, no structure)."}),"\n",(0,s.jsx)(t.h3,{id:"why-use-moon",children:"Why use moon?"}),"\n",(0,s.jsx)(t.p,{children:"Working in a language's ecosystem can be very involved, especially when it comes to managing a\nrepository effectively. Which language version to use? Which dependency manager to use? How to use\npackages? Or how to build packages? So on and so forth. moon aims to streamline this entire process\nand provide a first-class developer experience."}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Increased productivity"})," - With ",(0,s.jsx)(t.a,{href:"https://www.rust-lang.org/",children:"Rust"})," as our foundation, we can\nensure robust speeds, high performance, and low memory usage. Instead of long builds blocking you,\nfocus on your work."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Exceptional developer experience"})," - As veterans of developer tooling, we're well aware of the\npain points and frustrations. Our goal is to mitigate and overcome these obstacles."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Incremental adoption"})," - At its core, moon has been designed to be adopted incrementally and is\n",(0,s.jsx)(t.em,{children:"not"}),' an "all at once adoption". Migrate project-by-project, or task-by-task, it\'s up to you!']}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Reduced tasks confusion"})," - Tasks (for example, ",(0,s.jsx)(t.code,{children:"package.json"})," scripts) can become unwieldy,\nvery quickly. No more duplicating the same task into every project, or reverse-engineering which\nroot scripts to use. With moon, all you need to know is the project name, and a task name."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Ensure correct versions"})," - Whether it's a programming language or dependency manager, ensure\nthe same version of each tool is the same across ",(0,s.jsx)(t.em,{children:"every"})," developer's environment. No more wasted\nhours of debugging."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Automation built-in"})," - When applicable, moon will automatically install dependencies\n(",(0,s.jsx)(t.code,{children:"node_modules"}),"), or\n",(0,s.jsx)(t.a,{href:"/docs/config/toolchain#syncprojectworkspacedependencies",children:"sync project dependencies"}),", or even\n",(0,s.jsx)(t.a,{href:"/docs/config/toolchain#syncprojectreferences",children:"sync TypeScript project references"}),"."]}),"\n",(0,s.jsxs)(t.li,{children:["And of course, the amazing list of ",(0,s.jsx)(t.a,{href:"#features",children:"features"})," below!"]}),"\n"]}),"\n",(0,s.jsx)(t.h3,{id:"supported-languages",children:"Supported languages"}),"\n",(0,s.jsx)(t.p,{children:"moon's long-term vision is to robustly support multiple programming languages (and dependency\nmanagers) so that a repository composed of projects with differing languages and tools can all work\nin unison. This is a lofty vision that requires a massive amount of time and resources to achieve,\nand as such, is not available on initial release, but will gradually be supported over time."}),"\n",(0,s.jsx)(t.p,{children:"To help achieve this vision, language support is broken down into 4 tiers, allowing us to\nincrementally integrate and improve them over time. The 4 tiers are as follows:"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:["\xa0",(0,s.jsx)(l.Z,{text:"Tier 0",variant:"failure"})," \xa0 ",(0,s.jsx)(t.strong,{children:"No direct integration"})," - Tool is not\ndirectly supported in moon, but can still be ran using the\n",(0,s.jsx)(t.a,{href:"/docs/faq#can-we-run-other-languages",children:'"system" task toolchain'}),", which expects the tool to exist\nin the current environment."]}),"\n",(0,s.jsxs)(t.li,{children:["\xa0",(0,s.jsx)(l.Z,{text:"Tier 1",variant:"warning"})," \xa0 ",(0,s.jsx)(t.strong,{children:"Project categorization"})," - Projects can\nconfigure their primary ",(0,s.jsxs)(t.a,{href:"/docs/config/project#language",children:["language in ",(0,s.jsx)(t.code,{children:"moon.yml"})]}),", and have a\ndedicated Rust crate for metadata."]}),"\n",(0,s.jsxs)(t.li,{children:["\xa0",(0,s.jsx)(l.Z,{text:"Tier 2",variant:"info"})," \xa0 ",(0,s.jsx)(t.strong,{children:"Ecosystem platformization"})," - moon deeply\nintegrates with the language's ecosystem by parsing manifests, lockfiles, and other semantic files\nto infer dependencies, tasks, and other necessary information."]}),"\n",(0,s.jsxs)(t.li,{children:["\xa0",(0,s.jsx)(l.Z,{text:"Tier 3",variant:"success"})," \xa0 ",(0,s.jsx)(t.strong,{children:"Toolchain integration"})," - Language is\ndirectly supported in the toolchain, configured in\n",(0,s.jsx)(t.a,{href:"/docs/config/toolchain",children:(0,s.jsx)(t.code,{children:".moon/toolchain.yml"})}),", and will automatically be downloaded and\ninstalled."]}),"\n"]}),"\n",(0,s.jsxs)(t.table,{children:[(0,s.jsx)(t.thead,{children:(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.th,{style:{textAlign:"left"}}),(0,s.jsx)(t.th,{style:{textAlign:"center"},children:"Tier 0"}),(0,s.jsx)(t.th,{style:{textAlign:"center"},children:"Tier 1"}),(0,s.jsx)(t.th,{style:{textAlign:"center"},children:"Tier 2"}),(0,s.jsx)(t.th,{style:{textAlign:"center"},children:"Tier 3"})]})}),(0,s.jsxs)(t.tbody,{children:[(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Bash/Batch"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"}}),(0,s.jsx)(t.td,{style:{textAlign:"center"}})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Bun (JavaScript, TypeScript)"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Deno (JavaScript, TypeScript)"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Go"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"}}),(0,s.jsx)(t.td,{style:{textAlign:"center"}})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Node (JavaScript, TypeScript)"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"\u2514\u2500 npm, pnpm, yarn"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\u26aa\ufe0f"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"PHP"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"}}),(0,s.jsx)(t.td,{style:{textAlign:"center"}})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"\u2514\u2500 Composer"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\u26aa\ufe0f"}),(0,s.jsx)(t.td,{style:{textAlign:"center"}}),(0,s.jsx)(t.td,{style:{textAlign:"center"}})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Python"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"\u2514\u2500 Pip"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\u26aa\ufe0f"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"}),(0,s.jsx)(t.td,{style:{textAlign:"center"}})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Ruby"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"}}),(0,s.jsx)(t.td,{style:{textAlign:"center"}})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"\u2514\u2500 Gems, Bundler"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\u26aa\ufe0f"}),(0,s.jsx)(t.td,{style:{textAlign:"center"}}),(0,s.jsx)(t.td,{style:{textAlign:"center"}})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Rust"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"\u2514\u2500 Cargo"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\u26aa\ufe0f"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe3"})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Other (Kotlin, Java, C#, ...)"}),(0,s.jsx)(t.td,{style:{textAlign:"center"},children:"\ud83d\udfe2"}),(0,s.jsx)(t.td,{style:{textAlign:"center"}}),(0,s.jsx)(t.td,{style:{textAlign:"center"}}),(0,s.jsx)(t.td,{style:{textAlign:"center"}})]})]})]}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsx)(t.li,{children:"\u26aa\ufe0f Not applicable"}),"\n",(0,s.jsx)(t.li,{children:"\ud83d\udfe3 Partially supported (experimental)"}),"\n",(0,s.jsx)(t.li,{children:"\ud83d\udfe2 Fully Supported"}),"\n"]}),"\n",(0,s.jsx)(t.h3,{id:"supported-targets",children:"Supported targets"}),"\n",(0,s.jsx)(t.p,{children:"Because moon is written in Rust, we only support targets that are explicitly compiled for, which are\ncurrently:"}),"\n",(0,s.jsxs)(t.table,{children:[(0,s.jsx)(t.thead,{children:(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.th,{style:{textAlign:"left"},children:"Operating system"}),(0,s.jsx)(t.th,{style:{textAlign:"left"},children:"Architecture"}),(0,s.jsx)(t.th,{style:{textAlign:"left"},children:"Target"})]})}),(0,s.jsxs)(t.tbody,{children:[(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"macOS 64-bit"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Intel"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:(0,s.jsx)(t.code,{children:"x86_64-apple-darwin"})})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"macOS 64-bit"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"ARM"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:(0,s.jsx)(t.code,{children:"aarch64-apple-darwin"})})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Linux 64-bit"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Intel GNU"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:(0,s.jsx)(t.code,{children:"x86_64-unknown-linux-gnu"})})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Linux 64-bit"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Intel musl"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:(0,s.jsx)(t.code,{children:"x86_64-unknown-linux-musl"})})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Linux 64-bit"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"ARM GNU"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:(0,s.jsx)(t.code,{children:"aarch64-unknown-linux-gnu"})})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Linux 64-bit"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"ARM musl"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:(0,s.jsx)(t.code,{children:"aarch64-unknown-linux-musl"})})]}),(0,s.jsxs)(t.tr,{children:[(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Windows 64-bit"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:"Intel"}),(0,s.jsx)(t.td,{style:{textAlign:"left"},children:(0,s.jsx)(t.code,{children:"x86_64-pc-windows-msvc"})})]})]})]}),"\n",(0,s.jsx)(t.h3,{id:"features",children:"Features"}),"\n",(0,s.jsx)(t.h4,{id:"management",children:"Management"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Smart hashing"})," - Collects inputs from multiple sources to ensure builds are deterministic and\nreproducible."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Remote caching"})," - Persists builds, hashes, and caches between teammates and CI/CD environments."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Integrated toolchain"})," - Automatically downloads and installs explicit versions of Node.js and\nother tools for consistency across the entire workspace or per project."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Multi-platform"})," - Runs on common development platforms: Linux, macOS, and Windows."]}),"\n"]}),"\n",(0,s.jsx)(t.h4,{id:"organization",children:"Organization"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Project graph"})," - Generates a project graph for dependency and dependent relationships."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Code generation"})," - Easily scaffold new applications, libraries, tooling, and more!"]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Dependency workspaces"})," - Works alongside package manager workspaces so that projects have\ndistinct dependency trees."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Code ownership"})," - Declare owners, maintainers, support channels, and more. Generate CODEOWNERS."]}),"\n"]}),"\n",(0,s.jsx)(t.h4,{id:"orchestration",children:"Orchestration"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Dependency graph"})," - Generates a dependency graph to increase performance and reduce workloads."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Action pipeline"})," - Executes actions in parallel and in order using a thread pool and our\ndependency graph."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Action distribution"})," ",(0,s.jsx)(r.Z,{status:"coming-soon"})," - Distributes actions across\nmultiple machines to increase throughput."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Incremental builds"})," - With our smart hashing, only rebuild projects that have been touched\nsince the last build."]}),"\n"]}),"\n",(0,s.jsx)(t.h4,{id:"notification",children:"Notification"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Flakiness detection"})," - Reduce flaky builds with automatic retries and passthrough settings."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Webhook events"})," ",(0,s.jsx)(r.Z,{status:"experimental"})," - Receive a webhook for every event in\nthe pipeline. Useful for metrics gathering and insights."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Terminal notifications"})," ",(0,s.jsx)(r.Z,{status:"coming-soon"})," - Receives notifications in your\nchosen terminal when builds are successful... or are not."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"Git hooks"})," - Manage Git hooks to enforce workflows and requirements for contributors."]}),"\n"]}),"\n",(0,s.jsx)(t.h2,{id:"moonbase",children:"moonbase"}),"\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.a,{href:"/moonbase",children:"moonbase"})," is a service for gathering insights into your CI pipelines for ",(0,s.jsx)(t.a,{href:"#moon",children:"moon"}),"\npowered repositories. It offers remote caching of build artifacts to greatly reduce CI times,\ntracking of CI jobs to detect flakiness and regressions, project and code ownership registries,\nrepository health, and much more!"]}),"\n",(0,s.jsx)(t.p,{children:"This service is currently in heavy development."}),"\n",(0,s.jsx)(t.h2,{id:"proto",children:"proto"}),"\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.a,{href:"/proto",children:"proto"})," is a version manager for your favorite programming languages.\n",(0,s.jsx)(t.a,{href:"/docs/proto",children:"View proto documentation"}),"."]})]})}function g(e={}){const{wrapper:t}={...(0,i.a)(),...e.components};return t?(0,s.jsx)(t,{...e,children:(0,s.jsx)(x,{...e})}):x(e)}},59220:(e,t,n)=>{n.d(t,{Z:()=>r});var s=n(9619),i=n(24246);function r(e){let{className:t,status:n}=e;switch(n){case"experimental":return(0,i.jsx)(s.Z,{className:t,text:"Experimental",variant:"failure"});case"in-development":return(0,i.jsx)(s.Z,{className:t,text:"In development",variant:"success"});case"coming-soon":return(0,i.jsx)(s.Z,{className:t,text:"Coming soon",variant:"warning"});case"new":return(0,i.jsx)(s.Z,{className:t,text:"New",variant:"info"});default:return null}}},9619:(e,t,n)=>{n.d(t,{Z:()=>o});var s=n(40624),i=n(31792),r=n(24246);const l={failure:"bg-red-100 text-red-900",info:"bg-pink-100 text-pink-900",success:"bg-green-100 text-green-900",warning:"bg-orange-100 text-orange-900"};function o(e){let{className:t,icon:n,text:o,variant:c}=e;return(0,r.jsxs)("span",{className:(0,s.Z)("inline-flex items-center px-1 py-0.5 rounded text-xs font-bold uppercase",c?l[c]:"bg-gray-100 text-gray-800",t),children:[n&&(0,r.jsx)(i.Z,{icon:n,className:"mr-1"}),o]})}},71670:(e,t,n)=>{n.d(t,{Z:()=>o,a:()=>l});var s=n(27378);const i={},r=s.createContext(i);function l(e){const t=s.useContext(r);return s.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function o(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:l(e.components),s.createElement(r.Provider,{value:t},e.children)}}}]);