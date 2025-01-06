"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[28129],{92543:(e,n,s)=>{s.r(n),s.d(n,{assets:()=>a,contentTitle:()=>r,default:()=>d,frontMatter:()=>o,metadata:()=>c,toc:()=>h});var i=s(24246),t=s(71670);const o={title:"Debugging a task"},r=void 0,c={id:"guides/debug-task",title:"Debugging a task",description:"Running tasks is the most common way to interact with moon, so what do you do",source:"@site/docs/guides/debug-task.mdx",sourceDirName:"guides",slug:"/guides/debug-task",permalink:"/docs/guides/debug-task",draft:!1,unlisted:!1,editUrl:"https://github.com/moonrepo/moon/tree/master/website/docs/guides/debug-task.mdx",tags:[],version:"current",frontMatter:{title:"Debugging a task"},sidebar:"guides",previous:{title:"Code owners",permalink:"/docs/guides/codeowners"},next:{title:"Docker integration",permalink:"/docs/guides/docker"}},a={},h=[{value:"Verify configuration",id:"verify-configuration",level:2},{value:"Verify inherited configuration",id:"verify-inherited-configuration",level:3},{value:"Inspect trace logs",id:"inspect-trace-logs",level:2},{value:"Inspect the hash manifest",id:"inspect-the-hash-manifest",level:2},{value:"Diffing a previous hash",id:"diffing-a-previous-hash",level:3},{value:"Ask for help",id:"ask-for-help",level:2}];function l(e){const n={a:"a",admonition:"admonition",code:"code",em:"em",h2:"h2",h3:"h3",li:"li",ol:"ol",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,t.a)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsxs)(n.p,{children:["Running ",(0,i.jsx)(n.a,{href:"../concepts/task",children:"tasks"})," is the most common way to interact with moon, so what do you do\nwhen your task isn't working as expected? Diagnose it of course! Diagnosing the root cause of a\nbroken task can be quite daunting, but do not fret, as the following steps will help guide you in\nthis endeavor."]}),"\n",(0,i.jsx)(n.h2,{id:"verify-configuration",children:"Verify configuration"}),"\n",(0,i.jsxs)(n.p,{children:["Before we dive into the internals of moon, we should first verify that the task is actually\nconfigured correctly. Our configuration layer is very strict, but it can't catch everything, so jump\nto the ",(0,i.jsx)(n.a,{href:"../config/project#tasks",children:(0,i.jsx)(n.code,{children:"moon.yml"})})," documentation for more information."]}),"\n",(0,i.jsxs)(n.p,{children:["To start, moon will create a snapshot of the project and its tasks, with all ",(0,i.jsx)(n.a,{href:"../concepts/token",children:"tokens"}),"\nresolved, and paths expanded. This snapshot is located at\n",(0,i.jsx)(n.code,{children:".moon/cache/states/<project>/snapshot.json"}),". With the snapshot open, inspect the root ",(0,i.jsx)(n.code,{children:"tasks"}),"\nobject for any inconsistencies or inaccuracies."]}),"\n",(0,i.jsx)(n.p,{children:"Some issues to look out for:"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:["Have ",(0,i.jsx)(n.code,{children:"command"})," and ",(0,i.jsx)(n.code,{children:"args"})," been parsed correctly?"]}),"\n",(0,i.jsxs)(n.li,{children:["Have ",(0,i.jsx)(n.a,{href:"../concepts/token",children:"tokens"})," resolved correctly? If not, verify syntax or try another token type."]}),"\n",(0,i.jsxs)(n.li,{children:["Have ",(0,i.jsx)(n.code,{children:"inputFiles"}),", ",(0,i.jsx)(n.code,{children:"inputGlobs"}),", and ",(0,i.jsx)(n.code,{children:"inputVars"})," expanded correctly from ",(0,i.jsx)(n.a,{href:"../config/project#inputs",children:(0,i.jsx)(n.code,{children:"inputs"})}),"?"]}),"\n",(0,i.jsxs)(n.li,{children:["Have ",(0,i.jsx)(n.code,{children:"outputFiles"})," and ",(0,i.jsx)(n.code,{children:"outputGlobs"})," expanded correctly from ",(0,i.jsx)(n.a,{href:"../config/project#outputs",children:(0,i.jsx)(n.code,{children:"outputs"})}),"?"]}),"\n",(0,i.jsxs)(n.li,{children:["Is the ",(0,i.jsx)(n.code,{children:"toolchain"})," (formerly ",(0,i.jsx)(n.code,{children:"platform"}),") correct for the command? If incorrect, explicitly set the\n",(0,i.jsx)(n.a,{href:"../config/project#toolchain",children:(0,i.jsx)(n.code,{children:"toolchain"})}),"."]}),"\n",(0,i.jsxs)(n.li,{children:["Are ",(0,i.jsx)(n.code,{children:"options"})," and ",(0,i.jsx)(n.code,{children:"flags"})," correct?"]}),"\n"]}),"\n",(0,i.jsx)(n.admonition,{type:"info",children:(0,i.jsxs)(n.p,{children:["Resolved information can also be inspected with the ",(0,i.jsx)(n.a,{href:"../commands/task",children:(0,i.jsx)(n.code,{children:"moon task <target> --json"})}),"\ncommand."]})}),"\n",(0,i.jsx)(n.h3,{id:"verify-inherited-configuration",children:"Verify inherited configuration"}),"\n",(0,i.jsxs)(n.p,{children:["If the configuration from the previous step looks correct, you can skip this step, otherwise let's\nverify that the inherited configuration is also correct. In the ",(0,i.jsx)(n.code,{children:"snapshot.json"})," file, inspect the\nroot ",(0,i.jsx)(n.code,{children:"inherited"})," object, which is structured as follows:"]}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"order"})," - The order in which configuration files from ",(0,i.jsx)(n.code,{children:".moon"})," are loaded, from lowest to highest\npriority, and the order files are merged. The ",(0,i.jsx)(n.code,{children:"*"})," entry is ",(0,i.jsx)(n.code,{children:".moon/tasks.yml"}),", while other entries\nmap to ",(0,i.jsx)(n.code,{children:".moon/tasks/**/*.yml"}),"."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"layers"})," - A mapping of configuration files that were loaded, derived from the ",(0,i.jsx)(n.code,{children:"order"}),". Each layer\nrepresents a partial object (not expanded or resolved). Only files that exist will be mapped here."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"config"})," - A partial configuration object representing the state of all merged layers. This is\nwhat is merged with the project's ",(0,i.jsx)(n.code,{children:"moon.yml"})," file."]}),"\n"]}),"\n",(0,i.jsx)(n.p,{children:"Some issues to look out for:"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:["Is the order correct? If not, verify the project's ",(0,i.jsx)(n.a,{href:"../config/project#language",children:(0,i.jsx)(n.code,{children:"language"})})," and\nthe task's ",(0,i.jsx)(n.a,{href:"../config/project#toolchain",children:(0,i.jsx)(n.code,{children:"toolchain"})}),"."]}),"\n",(0,i.jsxs)(n.li,{children:["Does ",(0,i.jsx)(n.code,{children:"config"})," correctly represent the merged state of all ",(0,i.jsx)(n.code,{children:"layers"}),"? Do note that tasks are shallow\nmerged (by name), ",(0,i.jsx)(n.em,{children:"not"})," deep merged."]}),"\n",(0,i.jsxs)(n.li,{children:["Have the root ",(0,i.jsx)(n.code,{children:"tasks"})," properly inherited ",(0,i.jsx)(n.a,{href:"../config/tasks#implicitdeps",children:(0,i.jsx)(n.code,{children:"implicitDeps"})}),",\n",(0,i.jsx)(n.a,{href:"../config/tasks#implicitinputs",children:(0,i.jsx)(n.code,{children:"implicitInputs"})}),", and ",(0,i.jsx)(n.code,{children:"fileGroups"}),"?"]}),"\n"]}),"\n",(0,i.jsx)(n.h2,{id:"inspect-trace-logs",children:"Inspect trace logs"}),"\n",(0,i.jsx)(n.p,{children:"If configuration looks good, let's move on to inspecting the trace logs, which can be a non-trivial\namount of effort. Run the task to generate the logs, bypass the cache, and include debug\ninformation:"}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-shell",children:"MOON_DEBUG_PROCESS_ENV=true MOON_DEBUG_PROCESS_INPUT=true moon run <target> --log trace --updateCache\n"})}),"\n",(0,i.jsx)(n.p,{children:'Once ran, a large amount of information will be logged to the terminal. However, most of it can be\nignored, as we\'re only interested in the "is this task affected by changes" logs. This breaks down\nas follows:'}),"\n",(0,i.jsxs)(n.ol,{children:["\n",(0,i.jsxs)(n.li,{children:["First, we gather touched files from the local checkout, which is typically\n",(0,i.jsx)(n.code,{children:"git status --porcelain --untracked-files"})," (from the ",(0,i.jsx)(n.code,{children:"moon_process::command_inspector"})," module).\nThe logs do not output the list of files that are touched, but you can run this command locally\nto verify the output."]}),"\n",(0,i.jsxs)(n.li,{children:["Secondly, we gather all files from the project directory, using the\n",(0,i.jsx)(n.code,{children:"git ls-files --full-name --cached --modified --others --exclude-standard <path> --deduplicate"}),"\ncommand (also from the ",(0,i.jsx)(n.code,{children:"moon_process::command_inspector"})," module). This command can also be ran\nlocally to verify the output."]}),"\n",(0,i.jsxs)(n.li,{children:["Lastly, all files from the previous 2 commands will be hashed using the ",(0,i.jsx)(n.code,{children:"git hash-object"}),"\ncommand. If you passed the ",(0,i.jsx)(n.code,{children:"MOON_DEBUG_PROCESS_INPUT"})," environment variable, you'll see a massive\nlog entry of all files being hashed. This is what we use to generate moon's specific hash."]}),"\n"]}),"\n",(0,i.jsx)(n.p,{children:"If all went well, you should see a log entry that looks like this:"}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{children:"Generated hash <hash> for target <target>\n"})}),"\n",(0,i.jsx)(n.p,{children:"The important piece is the hash, which is a 64-character SHA256 hash, and represents the unique hash\nof this task/target. This is what moon uses to determine a cache hit/miss, and whether or not to\nskip re-running a task."}),"\n",(0,i.jsx)(n.p,{children:"Let's copy the hash and move on to the next step."}),"\n",(0,i.jsx)(n.h2,{id:"inspect-the-hash-manifest",children:"Inspect the hash manifest"}),"\n",(0,i.jsxs)(n.p,{children:["With the hash in hand, let's dig deeper into moon's internals, by inspecting the hash manifest at\n",(0,i.jsx)(n.code,{children:".moon/cache/hashes/<hash>.json"}),", or running the ",(0,i.jsx)(n.a,{href:"../commands/query/hash",children:(0,i.jsx)(n.code,{children:"moon query hash"})}),"\ncommand:"]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-shell",children:"moon query hash <hash>\n"})}),"\n",(0,i.jsx)(n.p,{children:"The manifest is JSON and its contents are all the information used to generate its unique hash. This\ninformation is an array, and breaks down as follows:"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:["The first item in the array is the task itself. The important fields to diagnose here are ",(0,i.jsx)(n.code,{children:"deps"}),"\nand ",(0,i.jsx)(n.code,{children:"inputs"}),".","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsx)(n.li,{children:"Dependencies are other tasks (and their hash) that this task depends on."}),"\n",(0,i.jsxs)(n.li,{children:["Inputs are all the files (and their hash from ",(0,i.jsx)(n.code,{children:"git hash-object"}),") this task requires to run."]}),"\n"]}),"\n"]}),"\n",(0,i.jsxs)(n.li,{children:["The remaining items are toolchain/language specific, some examples are:","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.strong,{children:"Node.js"})," - The current Node.js version and the resolved versions/hashes of all ",(0,i.jsx)(n.code,{children:"package.json"}),"\ndependencies."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.strong,{children:"Rust"})," - The current Rust version and the resolved versions/hashes of all ",(0,i.jsx)(n.code,{children:"Cargo.toml"}),"\ndependencies."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.strong,{children:"TypeScript"})," - Compiler options for changing compilation output."]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,i.jsx)(n.p,{children:"Some issues to look out for:"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:["Do the dependencies match the task's configured ",(0,i.jsx)(n.a,{href:"../config/project#deps",children:(0,i.jsx)(n.code,{children:"deps"})})," and ",(0,i.jsx)(n.a,{href:"../config/tasks#implicitdeps",children:(0,i.jsx)(n.code,{children:"implicitDeps"})}),"?"]}),"\n",(0,i.jsxs)(n.li,{children:["Do the inputs match the task's configured ",(0,i.jsx)(n.a,{href:"../config/project#inputs",children:(0,i.jsx)(n.code,{children:"inputs"})})," and\n",(0,i.jsx)(n.a,{href:"../config/tasks#implicitinputs",children:(0,i.jsx)(n.code,{children:"implicitInputs"})}),"? If not, try tweaking the config."]}),"\n",(0,i.jsx)(n.li,{children:"Are the toolchain/language specific items correct?"}),"\n",(0,i.jsx)(n.li,{children:"Are dependency versions/hashes correctly parsed from the appropriate lockfile?"}),"\n"]}),"\n",(0,i.jsx)(n.h3,{id:"diffing-a-previous-hash",children:"Diffing a previous hash"}),"\n",(0,i.jsxs)(n.p,{children:["Another avenue for diagnosing a task is to diff the hash against a hash from a previous run. Since\nwe require multiple hashes, we'll need to run the task multiple times,\n",(0,i.jsx)(n.a,{href:"#inspect-trace-logs",children:"inspect the logs"}),", and extract the hash for each. If you receive the same hash\nfor each run, you'll need to tweak configuration or change files to produce a different hash."]}),"\n",(0,i.jsxs)(n.p,{children:["Once you have 2 unique hashes, we can pass them to the\n",(0,i.jsx)(n.a,{href:"../commands/query/hash-diff",children:(0,i.jsx)(n.code,{children:"moon query hash-diff"})})," command. This will produce a ",(0,i.jsx)(n.code,{children:"git diff"})," styled\noutput, allowing for simple line-by-line comparison debugging."]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-shell",children:"moon query hash-diff <hash-left> <hash-right>\n"})}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-diff",children:'Left:  0b55b234f1018581c45b00241d7340dc648c63e639fbafdaf85a4cd7e718fdde\nRight: 2388552fee5a02062d0ef402bdc7232f0a447458b058c80ce9c3d0d4d7cfe171\n\n[\n\t{\n\t\t"command": "build",\n\t\t"args": [\n+\t\t\t"./dist"\n-\t\t\t"./build"\n\t\t],\n\t\t...\n\t}\n]\n'})}),"\n",(0,i.jsx)(n.p,{children:"This is extremely useful in diagnoising why a task is running differently than before, and is much\neasier than inspecting the hash manifest files manually!"}),"\n",(0,i.jsx)(n.h2,{id:"ask-for-help",children:"Ask for help"}),"\n",(0,i.jsx)(n.p,{children:"If you've made it this far, and still can't figure out why a task is not working correctly, please\nask for help!"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.a,{href:"https://discord.gg/qCh9MEynv2",children:"Join the Discord community"})," (if lost)"]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.a,{href:"https://github.com/moonrepo/moon/issues/new/choose",children:"Report an issue"})," (if an actual bug)"]}),"\n"]})]})}function d(e={}){const{wrapper:n}={...(0,t.a)(),...e.components};return n?(0,i.jsx)(n,{...e,children:(0,i.jsx)(l,{...e})}):l(e)}},71670:(e,n,s)=>{s.d(n,{Z:()=>c,a:()=>r});var i=s(27378);const t={},o=i.createContext(t);function r(e){const n=i.useContext(o);return i.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function c(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(t):e.components||t:r(e.components),i.createElement(o.Provider,{value:n},e.children)}}}]);