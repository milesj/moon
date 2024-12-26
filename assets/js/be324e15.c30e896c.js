"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[9847],{40073:(e,n,o)=>{o.r(n),o.d(n,{assets:()=>d,contentTitle:()=>c,default:()=>u,frontMatter:()=>i,metadata:()=>a,toc:()=>l});var t=o(24246),r=o(71670),s=o(79022);const i={title:"upgrade"},c=void 0,a={id:"proto/commands/upgrade",title:"upgrade",description:"The proto upgrade (or proto up) command can be used to upgrade your current proto binary to the",source:"@site/docs/proto/commands/upgrade.mdx",sourceDirName:"proto/commands",slug:"/proto/commands/upgrade",permalink:"/docs/proto/commands/upgrade",draft:!1,unlisted:!1,editUrl:"https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/upgrade.mdx",tags:[],version:"current",frontMatter:{title:"upgrade"},sidebar:"proto",previous:{title:"unpin",permalink:"/docs/proto/commands/unpin"},next:{title:"versions",permalink:"/docs/proto/commands/versions"}},d={},l=[{value:"Arguments",id:"arguments",level:3},{value:"Options",id:"options",level:3}];function p(e){const n={admonition:"admonition",code:"code",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,r.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsxs)(n.p,{children:["The ",(0,t.jsx)(n.code,{children:"proto upgrade"})," (or ",(0,t.jsx)(n.code,{children:"proto up"}),") command can be used to upgrade your current proto binary to the\nlatest version, or check if you're currently outdated."]}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-shell",children:"$ proto upgrade\n\n# Up/downgrade to a specific version\n$ proto upgrade 0.39.0\n"})}),"\n",(0,t.jsx)(n.admonition,{type:"info",children:(0,t.jsxs)(n.p,{children:["The previous binary will be moved to ",(0,t.jsx)(n.code,{children:"~/.proto/tools/proto/<version>"}),", while the new binary will be\ninstalled to ",(0,t.jsx)(n.code,{children:"~/.proto/bin"}),"."]})}),"\n",(0,t.jsx)(n.h3,{id:"arguments",children:"Arguments"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"<version>"})," - The version of proto to explicitly upgrade or downgrade to.","\n",(0,t.jsx)(s.Z,{version:"0.39.3"}),"\n"]}),"\n"]}),"\n",(0,t.jsx)(n.h3,{id:"options",children:"Options"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"--check"})," - Check if there's a new version without executing the upgrade."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"--json"})," - Print the upgrade information as JSON."]}),"\n"]})]})}function u(e={}){const{wrapper:n}={...(0,r.a)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(p,{...e})}):p(e)}},79022:(e,n,o)=>{o.d(n,{Z:()=>s});var t=o(9619),r=o(24246);function s(e){let{header:n,inline:o,updated:s,version:i}=e;return(0,r.jsx)(t.Z,{text:`v${i}`,variant:s?"success":"info",className:n?"absolute right-0 top-1.5":o?"inline-block":"ml-2"})}},9619:(e,n,o)=>{o.d(n,{Z:()=>c});var t=o(40624),r=o(31792),s=o(24246);const i={failure:"bg-red-100 text-red-900",info:"bg-pink-100 text-pink-900",success:"bg-green-100 text-green-900",warning:"bg-orange-100 text-orange-900"};function c(e){let{className:n,icon:o,text:c,variant:a}=e;return(0,s.jsxs)("span",{className:(0,t.Z)("inline-flex items-center px-1 py-0.5 rounded text-xs font-bold uppercase",a?i[a]:"bg-gray-100 text-gray-800",n),children:[o&&(0,s.jsx)(r.Z,{icon:o,className:"mr-1"}),c]})}},71670:(e,n,o)=>{o.d(n,{Z:()=>c,a:()=>i});var t=o(27378);const r={},s=t.createContext(r);function i(e){const n=t.useContext(s);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function c(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:i(e.components),t.createElement(s.Provider,{value:n},e.children)}}}]);