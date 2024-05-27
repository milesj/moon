"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[82536],{85138:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>c,contentTitle:()=>a,default:()=>d,frontMatter:()=>i,metadata:()=>r,toc:()=>l});var s=t(24246),o=t(71670);const i={slug:"moon-v1.25",title:"moon v1.25 - New task runner and console reporter",authors:["milesj"],tags:["task","runner","console","reporter","operation"],image:"./img/moon/v1.25.png"},a=void 0,r={permalink:"/blog/moon-v1.25",editUrl:"https://github.com/moonrepo/moon/tree/master/website/blog/2024-05-27_moon-v1.25.mdx",source:"@site/blog/2024-05-27_moon-v1.25.mdx",title:"moon v1.25 - New task runner and console reporter",description:"In this release, we focused primarily on rewriting our task runner, and improving our console.",date:"2024-05-27T00:00:00.000Z",tags:[{label:"task",permalink:"/blog/tags/task"},{label:"runner",permalink:"/blog/tags/runner"},{label:"console",permalink:"/blog/tags/console"},{label:"reporter",permalink:"/blog/tags/reporter"},{label:"operation",permalink:"/blog/tags/operation"}],readingTime:5.25,hasTruncateMarker:!0,authors:[{name:"Miles Johnson",title:"Founder, developer",url:"https://github.com/milesj",imageURL:"/img/authors/miles.jpg",key:"milesj"}],frontMatter:{slug:"moon-v1.25",title:"moon v1.25 - New task runner and console reporter",authors:["milesj"],tags:["task","runner","console","reporter","operation"],image:"./img/moon/v1.25.png"},unlisted:!1,nextItem:{title:"moon v1.24 - Task mutexes, auto-detect revisions, project dependents, and more!",permalink:"/blog/moon-v1.24"}},c={image:t(88776).Z,authorsImageUrls:[void 0]},l=[{value:"New task runner implementation",id:"new-task-runner-implementation",level:2},{value:"Fine-grained operations",id:"fine-grained-operations",level:3},{value:"Run summaries",id:"run-summaries",level:3},{value:"New console reporting layer",id:"new-console-reporting-layer",level:2},{value:"Other changes",id:"other-changes",level:2}];function h(e){const n={a:"a",blockquote:"blockquote",code:"code",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,o.a)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(n.p,{children:"In this release, we focused primarily on rewriting our task runner, and improving our console."}),"\n",(0,s.jsx)(n.h2,{id:"new-task-runner-implementation",children:"New task runner implementation"}),"\n",(0,s.jsx)(n.p,{children:"It's been over a month since our last release, but we've been really busy rewriting our task runner\nfrom the ground up! In other build systems, a task runner is typically the orchestator that runs\nmultiple tasks and manages their state. In moon this is known as the action pipeline (or just\npipeline), and a task runner is simply the execution of a single task. However, executing a single\ntask is quite involved, as we need to generate a unique hash, check the cache, hydrate outputs if a\ncache hit, actually execute the task as a child process, and much more!"}),"\n",(0,s.jsx)(n.p,{children:"Task running is some of the oldest code in moon, as it was part of the initial MVP. Because of this,\nit hasn't changed much, but moon has grown quite large and it was time to revisit it with better\ndesign patterns and practices. Furthermore, since the task runner is so critical to moon itself, we\nwanted to ensure it worked correctly, and spent more time than usual implementing, testing it, and\nverifying edge cases."}),"\n",(0,s.jsx)(n.p,{children:"With this new task runner, we..."}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"Improved handling and reliability of output archiving (cache miss) and hydration (cache hit)."}),"\n",(0,s.jsx)(n.li,{children:"Streamlined the task execution (child process) flow."}),"\n",(0,s.jsx)(n.li,{children:"Increased performance by optimizing or removing certain code paths."}),"\n"]}),"\n",(0,s.jsxs)(n.blockquote,{children:["\n",(0,s.jsxs)(n.p,{children:["If you're interested in how the task runner was implemented, feel free to take a look at the\n",(0,s.jsx)(n.a,{href:"https://github.com/moonrepo/moon/tree/master/nextgen/task-runner",children:"Rust crate"}),", and the\n",(0,s.jsx)(n.a,{href:"https://github.com/moonrepo/moon/pull/1463",children:"pull request"})," itself."]}),"\n"]}),"\n",(0,s.jsx)(n.h3,{id:"fine-grained-operations",children:"Fine-grained operations"}),"\n",(0,s.jsx)(n.p,{children:"A major goal of moon is bubbling up information to the user that is applicable to the current\nworkflow, but what about when that workflow must be debugged or optimized? At that point, it was\nalmost impossible without digging into the source code."}),"\n",(0,s.jsx)(n.p,{children:"To make a step in this direction, as part of the new task runner we now track timing information for\nindividual parts of the run execution, and we're calling these parts operations. An operation is\nanything from generating a hash, creating a tarball archive, unpacking the archive (cache\nhydration), task execution (the child process), and more."}),"\n",(0,s.jsxs)(n.p,{children:["This timing information is useful in figuring out why a certain task is slower than expected, and\nwhich operation is actually causing the slowness. It also helps to uncover which operations were\nactually ran for an action, which were skipped, so on and so forth. At this point in time, the\noperations information is only included in the run report, located at ",(0,s.jsx)(n.code,{children:".moon/cache/runReport.json"}),".\nIn the future, we plan to display this information in a nice UI."]}),"\n",(0,s.jsxs)(n.p,{children:["For an example of this in action, here's a list of all operations that were executed when running\nthe ",(0,s.jsx)(n.code,{children:"build"})," task for our website."]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-json",children:'[\n  {\n    "duration": {\n      "secs": 0,\n      "nanos": 609156875\n    },\n    "finishedAt": "2024-05-27T00:14:54.286628",\n    "meta": {\n      "type": "hash-generation",\n      "hash": "10606e37c5e6ab4008007b30275f1682bae32dca71650ce173eb386d5b6c3309"\n    },\n    "startedAt": "2024-05-27T00:14:53.677526",\n    "status": "passed"\n  },\n  {\n    "duration": {\n      "secs": 0,\n      "nanos": 32834\n    },\n    "finishedAt": "2024-05-27T00:14:54.286667",\n    "meta": {\n      "type": "output-hydration"\n    },\n    "startedAt": "2024-05-27T00:14:54.286634",\n    "status": "skipped"\n  },\n  {\n    "duration": {\n      "secs": 15,\n      "nanos": 789003125\n    },\n    "finishedAt": "2024-05-27T00:15:10.075113",\n    "meta": {\n      "type": "task-execution",\n      "command": "docusaurus build",\n      "exitCode": 0\n    },\n    "startedAt": "2024-05-27T00:14:54.286950",\n    "status": "passed"\n  },\n  {\n    "duration": {\n      "secs": 17,\n      "nanos": 214634292\n    },\n    "finishedAt": "2024-05-27T00:15:27.289995",\n    "meta": {\n      "type": "archive-creation"\n    },\n    "startedAt": "2024-05-27T00:15:10.075686",\n    "status": "passed"\n  }\n]\n'})}),"\n",(0,s.jsx)(n.p,{children:"Because of these new operations, we can clearly see above that the archive creation process is\ntaking 17 seconds, which is 2 seconds longer than the build itself! Without this information, we\nwould have never known that the archive was taking this long, but now we do, and we can optimize it\nin a future release!"}),"\n",(0,s.jsx)(n.h3,{id:"run-summaries",children:"Run summaries"}),"\n",(0,s.jsxs)(n.p,{children:["Because of the new task runner and the new console (",(0,s.jsx)(n.a,{href:"#new-console-reporting-layer",children:"below"}),"), we have\nthe ability to bubble up more information than before. Based on requests from the community, we've\ntaken the output from ",(0,s.jsx)(n.a,{href:"/docs/commands/ci",children:(0,s.jsx)(n.code,{children:"moon ci"})})," and applied it to both\n",(0,s.jsx)(n.a,{href:"/docs/commands/check",children:(0,s.jsx)(n.code,{children:"moon check"})})," and ",(0,s.jsx)(n.a,{href:"/docs/commands/run",children:(0,s.jsx)(n.code,{children:"moon run"})})," behind the ",(0,s.jsx)(n.code,{children:"--summary"}),"\nflag."]}),"\n",(0,s.jsxs)(n.p,{children:["When this flag is passed, we will now summarize all actions that have ran in the pipeline (not just\ntask related ones), and include failed tasks for review. For example, here's the output of\n",(0,s.jsx)(n.code,{children:"moon check website --summary"})," on moon's repository."]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:"$ moon check website --summary\n\n\u25aa\u25aa\u25aa\u25aa types:build (cached, 1ms, 21bd9add)\n\u25aa\u25aa\u25aa\u25aa runtime:build (cached, e8363e65)\n\u25aa\u25aa\u25aa\u25aa website:typecheck (cached, 0ab91eaa)\n\u25aa\u25aa\u25aa\u25aa website:format (cached, 07ae2388)\n\u25aa\u25aa\u25aa\u25aa website:test (cached, 11d33e2e)\n\u25aa\u25aa\u25aa\u25aa website:lint (cached, 2197fbb1)\n\u25aa\u25aa\u25aa\u25aa website:build (10606e37)\n\u25aa\u25aa\u25aa\u25aa website:build (15s 789ms, 10606e37)\n\nSUMMARY\n\npass SyncWorkspace\nskip SetupNodeTool(20.13.1) (skipped, 250ms)\nskip InstallNodeDeps(20.13.1) (skipped, 13ms, f341872f)\npass SyncNodeProject(types) (1ms)\npass SyncNodeProject(runtime) (1ms)\npass RunTask(types:build) (cached, 140ms, 21bd9add)\npass SyncNodeProject(website) (1ms)\npass RunTask(runtime:build) (cached, 32ms, e8363e65)\npass RunTask(website:build) (33s 614ms, 10606e37)\npass RunTask(website:format) (cached, 59ms, 07ae2388)\npass RunTask(website:lint) (cached, 101ms, 2197fbb1)\npass RunTask(website:test) (cached, 64ms, 11d33e2e)\npass RunTask(website:typecheck) (cached, 59ms, 0ab91eaa)\n\nSTATS\n\nActions: 11 completed (6 cached), 2 skipped\n   Time: 34s 52ms\n"})}),"\n",(0,s.jsx)(n.h2,{id:"new-console-reporting-layer",children:"New console reporting layer"}),"\n",(0,s.jsxs)(n.p,{children:["For the most part, when something in moon needed to be printed to the console, we would simply print\nit directly at that point in time, anywhere within the codebase. While this worked, it made it\ndifficult to orchestrate output from different parts of the codebase, and in the context of Rust,\neach call to stdout/stderr ",(0,s.jsx)(n.a,{href:"https://nnethercote.github.io/perf-book/io.html",children:"locks the I/O stream"}),",\nresulting in performance loss."]}),"\n",(0,s.jsxs)(n.p,{children:["To work around this, in ",(0,s.jsx)(n.a,{href:"./moon-v1.21",children:"v1.21"})," we implemented a new\n",(0,s.jsx)(n.a,{href:"https://github.com/moonrepo/moon/tree/master/nextgen/console",children:"console layer"})," that buffers all stdio\nwrites, and prints them on 100ms intervals. This avoids locking on every call, and instead batches\nthem. To expand on this further, in this release we've implemented a new\n",(0,s.jsx)(n.a,{href:"https://github.com/moonrepo/moon/tree/master/nextgen/console-reporter",children:"reporter layer"}),", with a\nwell-defined interface that is used to print checkpoints (the 4 squares), and status updates from\nthe action pipeline (and the new task runner)."]}),"\n",(0,s.jsxs)(n.p,{children:["This reporter layer enables new console UI implementations in the future based on your preferences.\nFor example, an ",(0,s.jsx)(n.a,{href:"https://ratatui.rs/",children:"interactive UI"})," composed of tables, tabs, and more,\nrepresenting the current state of the pipeline."]}),"\n",(0,s.jsx)(n.h2,{id:"other-changes",children:"Other changes"}),"\n",(0,s.jsxs)(n.p,{children:["View the ",(0,s.jsx)(n.a,{href:"https://github.com/moonrepo/moon/releases/tag/v1.25.0",children:"official release"})," for a full list\nof changes."]}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"Greatly reduced the amount of concurrent locks being held during task execution. May see slight\nperformance improvements."}),"\n",(0,s.jsx)(n.li,{children:"Updated external configuration files (via https extends) to be cached for 24 hours."}),"\n",(0,s.jsx)(n.li,{children:"Updated macOS binaries to be built on macos-12 instead of macos-11."}),"\n",(0,s.jsx)(n.li,{children:"Updated proto to v0.35.4 (from v0.34.4)."}),"\n"]})]})}function d(e={}){const{wrapper:n}={...(0,o.a)(),...e.components};return n?(0,s.jsx)(n,{...e,children:(0,s.jsx)(h,{...e})}):h(e)}},88776:(e,n,t)=>{t.d(n,{Z:()=>s});const s=t.p+"assets/images/v1.25-4fc2d5974abeac9266bcc8780720611f.png"},71670:(e,n,t)=>{t.d(n,{Z:()=>r,a:()=>a});var s=t(27378);const o={},i=s.createContext(o);function a(e){const n=s.useContext(i);return s.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function r(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(o):e.components||o:a(e.components),s.createElement(i.Provider,{value:n},e.children)}}}]);