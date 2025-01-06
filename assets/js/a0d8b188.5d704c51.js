"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[25547],{97199:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>p,contentTitle:()=>d,default:()=>u,frontMatter:()=>o,metadata:()=>c,toc:()=>h});var i=t(24246),a=t(71670),l=t(32189),r=t(9785),s=t(79022);const o={title:"template.yml",toc_max_heading_level:6},d=void 0,c={id:"config/template",title:"template.yml",description:"The template.yml file configures metadata and variables for a template,",source:"@site/docs/config/template.mdx",sourceDirName:"config",slug:"/config/template",permalink:"/docs/config/template",draft:!1,unlisted:!1,editUrl:"https://github.com/moonrepo/moon/tree/master/website/docs/config/template.mdx",tags:[],version:"current",frontMatter:{title:"template.yml",toc_max_heading_level:6},sidebar:"docs",previous:{title:"moon.yml",permalink:"/docs/config/project"},next:{title:"Editors",permalink:"/docs/editors"}},p={},h=[{value:"<code>id</code><VersionLabel></VersionLabel>",id:"id",level:2},{value:"<code>title</code><RequiredLabel></RequiredLabel>",id:"title",level:2},{value:"<code>description</code><RequiredLabel></RequiredLabel>",id:"description",level:2},{value:"<code>destination</code><VersionLabel></VersionLabel>",id:"destination",level:2},{value:"<code>extends</code><VersionLabel></VersionLabel>",id:"extends",level:2},{value:"<code>variables</code>",id:"variables",level:2},{value:"<code>type</code><RequiredLabel></RequiredLabel>",id:"type",level:3},{value:"<code>internal</code><VersionLabel></VersionLabel>",id:"internal",level:3},{value:"<code>order</code><VersionLabel></VersionLabel>",id:"order",level:3},{value:"Primitives",id:"primitives",level:3},{value:"<code>default</code><RequiredLabel></RequiredLabel>",id:"default",level:3},{value:"<code>prompt</code>",id:"prompt",level:3},{value:"<code>required</code>",id:"required",level:3},{value:"Enums",id:"enums",level:3},{value:"<code>default</code>",id:"default-1",level:3},{value:"<code>prompt</code>",id:"prompt-1",level:3},{value:"<code>multiple</code>",id:"multiple",level:3},{value:"<code>values</code><RequiredLabel></RequiredLabel>",id:"values",level:3},{value:"Frontmatter",id:"frontmatter",level:2},{value:"<code>force</code>",id:"force",level:3},{value:"<code>to</code>",id:"to",level:3},{value:"<code>skip</code>",id:"skip",level:3}];function m(e){const n={a:"a",blockquote:"blockquote",code:"code",em:"em",h2:"h2",h3:"h3",p:"p",pre:"pre",...(0,a.a)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsxs)(n.p,{children:["The ",(0,i.jsx)(n.code,{children:"template.yml"})," file configures metadata and variables for a template,\n",(0,i.jsx)(n.a,{href:"../guides/codegen",children:"used by the generator"}),", and must exist at the root of a named template folder."]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"$schema: 'https://moonrepo.dev/schemas/template.json'\n"})}),"\n",(0,i.jsxs)(n.h2,{id:"id",children:[(0,i.jsx)(n.code,{children:"id"}),(0,i.jsx)(s.Z,{version:"1.23.0"})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateConfig#id"}),"\n",(0,i.jsxs)(n.p,{children:["Overrides the name (identifier) of the template, instead of inferring the name from the template\nfolder. Be aware that template names must be unique across the workspace, and across all template\nlocations that have been configured in ",(0,i.jsx)(n.a,{href:"./workspace#templates",children:(0,i.jsx)(n.code,{children:"generator.templates"})}),"."]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"id: 'npm-package'\n"})}),"\n",(0,i.jsxs)(n.h2,{id:"title",children:[(0,i.jsx)(n.code,{children:"title"}),(0,i.jsx)(r.Z,{})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateConfig#title"}),"\n",(0,i.jsxs)(n.p,{children:["A human readable title that will be displayed during the ",(0,i.jsx)(n.a,{href:"../commands/generate",children:(0,i.jsx)(n.code,{children:"moon generate"})}),"\nprocess."]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"title: 'npm package'\n"})}),"\n",(0,i.jsxs)(n.h2,{id:"description",children:[(0,i.jsx)(n.code,{children:"description"}),(0,i.jsx)(r.Z,{})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateConfig#description"}),"\n",(0,i.jsx)(n.p,{children:"A description of why the template exists, what its purpose is, and any other relevant information."}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"description: |\n  Scaffolds the initial structure for an npm package,\n  including source and test folders, a package.json, and more.\n"})}),"\n",(0,i.jsxs)(n.h2,{id:"destination",children:[(0,i.jsx)(n.code,{children:"destination"}),(0,i.jsx)(s.Z,{version:"1.19.0"})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateConfig#destination"}),"\n",(0,i.jsxs)(n.p,{children:["An optional file path in which this template should be generated into. This provides a mechanism for\nstandardizing a destination location, and avoids having to manually pass a destination to\n",(0,i.jsx)(n.a,{href:"../commands/generate",children:(0,i.jsx)(n.code,{children:"moon generate"})}),"."]}),"\n",(0,i.jsxs)(n.p,{children:["If the destination is prefixed with ",(0,i.jsx)(n.code,{children:"/"}),", it will be relative from the workspace root, otherwise it\nis relative from the current working directory."]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"destination: 'packages/[name]'\n"})}),"\n",(0,i.jsxs)(n.blockquote,{children:["\n",(0,i.jsxs)(n.p,{children:["This setting supports ",(0,i.jsx)(n.a,{href:"#variables",children:"template variables"})," through ",(0,i.jsx)(n.code,{children:"[varName]"})," syntax. Learn more in\nthe ",(0,i.jsx)(n.a,{href:"../guides/codegen#interpolation",children:"code generation documentation"}),"."]}),"\n"]}),"\n",(0,i.jsxs)(n.h2,{id:"extends",children:[(0,i.jsx)(n.code,{children:"extends"}),(0,i.jsx)(s.Z,{version:"1.19.0"})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateConfig#extends"}),"\n",(0,i.jsx)(n.p,{children:"One or many other templates that this template should extend. Will deeply inherit all template files\nand variables."}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"extends: ['base', 'configs']\n"})}),"\n",(0,i.jsx)(n.h2,{id:"variables",children:(0,i.jsx)(n.code,{children:"variables"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateConfig#variables"}),"\n",(0,i.jsxs)(n.p,{children:["A mapping of variables that will be interpolated into all template files and file system paths when\n",(0,i.jsx)(n.a,{href:"https://tera.netlify.app/docs/#variables",children:"rendering with Tera"}),". The map key is the variable name\n(in camelCase or snake_case), while the value is a configuration object, as described with the\nsettings below."]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"variables:\n  name:\n    type: 'string'\n    default: ''\n    required: true\n    prompt: 'Package name?'\n"})}),"\n",(0,i.jsxs)(n.h3,{id:"type",children:[(0,i.jsx)(n.code,{children:"type"}),(0,i.jsx)(r.Z,{})]}),"\n",(0,i.jsxs)(n.p,{children:["The type of value for the variable. Accepts ",(0,i.jsx)(n.code,{children:"boolean"}),", ",(0,i.jsx)(n.code,{children:"string"}),", ",(0,i.jsx)(n.code,{children:"number"}),", or ",(0,i.jsx)(n.code,{children:"enum"}),". Floats ",(0,i.jsx)(n.em,{children:"are\nnot supported"}),", use strings instead."]}),"\n",(0,i.jsxs)(n.h3,{id:"internal",children:[(0,i.jsx)(n.code,{children:"internal"}),(0,i.jsx)(s.Z,{version:"1.23.0"})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateVariableStringSetting#internal"}),"\n",(0,i.jsx)(n.p,{children:"Marks a variable as internal only, which avoids the variable value being overwritten by command line\narguments."}),"\n",(0,i.jsxs)(n.h3,{id:"order",children:[(0,i.jsx)(n.code,{children:"order"}),(0,i.jsx)(s.Z,{version:"1.23.0"})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateVariableStringSetting#order"}),"\n",(0,i.jsxs)(n.p,{children:["The order in which the variable will be prompted to the user. By default, variables are prompted in\nthe order they are defined in the ",(0,i.jsx)(n.code,{children:"template.yml"})," file."]}),"\n",(0,i.jsx)(n.h3,{id:"primitives",children:"Primitives"}),"\n",(0,i.jsx)(n.p,{children:"Your basic primitives: boolean, numbers, strings."}),"\n",(0,i.jsxs)(n.h3,{id:"default",children:[(0,i.jsx)(n.code,{children:"default"}),(0,i.jsx)(r.Z,{})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateVariableStringSetting#default"}),"\n",(0,i.jsxs)(n.p,{children:["The default value of the variable. When ",(0,i.jsx)(n.code,{children:"--defaults"})," is passed to\n",(0,i.jsx)(n.a,{href:"../commands/generate",children:(0,i.jsx)(n.code,{children:"moon generate"})})," or ",(0,i.jsx)(n.a,{href:"#prompt",children:(0,i.jsx)(n.code,{children:"prompt"})})," is not defined, the default value\nwill be used, otherwise the user will be prompted to enter a custom value."]}),"\n",(0,i.jsx)(n.h3,{id:"prompt",children:(0,i.jsx)(n.code,{children:"prompt"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateVariableStringSetting#prompt"}),"\n",(0,i.jsxs)(n.p,{children:["When defined, will prompt the user with a message in the terminal to input a custom value, otherwise\n",(0,i.jsx)(n.a,{href:"#default",children:(0,i.jsx)(n.code,{children:"default"})})," will be used."]}),"\n",(0,i.jsx)(n.h3,{id:"required",children:(0,i.jsx)(n.code,{children:"required"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateVariableStringSetting#required"}),"\n",(0,i.jsxs)(n.p,{children:["Marks the variable as required during ",(0,i.jsx)(n.em,{children:"prompting only"}),". For strings, will error for empty values\n(",(0,i.jsx)(n.code,{children:"''"}),"). For numbers, will error for zero's (",(0,i.jsx)(n.code,{children:"0"}),")."]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"variables:\n  age:\n    type: 'number'\n    default: 0\n    required: true\n    prompt: 'Age?'\n"})}),"\n",(0,i.jsx)(n.h3,{id:"enums",children:"Enums"}),"\n",(0,i.jsx)(n.p,{children:"An enum is an explicit list of string values that a user can choose from."}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"variables:\n  color:\n    type: 'enum'\n    values: ['red', 'green', 'blue', 'purple']\n    default: 'purple'\n    prompt: 'Favorite color?'\n"})}),"\n",(0,i.jsx)(n.h3,{id:"default-1",children:(0,i.jsx)(n.code,{children:"default"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateVariableConfig#default"}),"\n",(0,i.jsxs)(n.p,{children:["The default value of the variable. When ",(0,i.jsx)(n.code,{children:"--defaults"})," is passed to\n",(0,i.jsx)(n.a,{href:"../commands/generate",children:(0,i.jsx)(n.code,{children:"moon generate"})})," or ",(0,i.jsx)(n.a,{href:"#prompt",children:(0,i.jsx)(n.code,{children:"prompt"})})," is not defined, the default value\nwill be used, otherwise the user will be prompted to enter a custom value."]}),"\n",(0,i.jsxs)(n.p,{children:["For enums, the default value can be a string when ",(0,i.jsx)(n.a,{href:"#multiple",children:(0,i.jsx)(n.code,{children:"multiple"})})," is false, or a string or\nan array of strings when ",(0,i.jsx)(n.code,{children:"multiple"})," is true. Furthermore, each default value must exist in the\n",(0,i.jsx)(n.a,{href:"#values",children:(0,i.jsx)(n.code,{children:"values"})})," list."]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"# Single\nvariables:\n  color:\n    type: 'enum'\n    values: ['red', 'green', 'blue', 'purple']\n    default: 'purple'\n    prompt: 'Favorite color?'\n\n# Multiple\nvariables:\n  color:\n    type: 'enum'\n    values: ['red', 'green', 'blue', 'purple']\n    default: ['red', 'purple']\n    multiple: true\n    prompt: 'Favorite color?'\n"})}),"\n",(0,i.jsx)(n.h3,{id:"prompt-1",children:(0,i.jsx)(n.code,{children:"prompt"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateVariableConfig#prompt"}),"\n",(0,i.jsxs)(n.p,{children:["When defined, will prompt the user with a message in the terminal to input a custom value, otherwise\n",(0,i.jsx)(n.a,{href:"#default",children:(0,i.jsx)(n.code,{children:"default"})})," will be used."]}),"\n",(0,i.jsx)(n.h3,{id:"multiple",children:(0,i.jsx)(n.code,{children:"multiple"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateEnumVariableConfig#multiple"}),"\n",(0,i.jsx)(n.p,{children:"Allows multiple values to be chosen during prompting. In the template, an array or strings will be\nrendered, otherwise when not-multiple, a single string will be."}),"\n",(0,i.jsxs)(n.h3,{id:"values",children:[(0,i.jsx)(n.code,{children:"values"}),(0,i.jsx)(r.Z,{})]}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateEnumVariableConfig#values"}),"\n",(0,i.jsx)(n.p,{children:"List of explicit values to choose from. Can either be defined with a string, which acts as a value\nand label, or as an object, which defines an explicit value and label."}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-yaml",metastring:'title="template.yml"',children:"variables:\n  color:\n    type: 'enum'\n    values:\n      - 'red'\n      # OR\n      - value: 'red'\n        label: 'Red \ud83d\udd34'\n    # ...\n"})}),"\n",(0,i.jsx)(n.h2,{id:"frontmatter",children:"Frontmatter"}),"\n",(0,i.jsxs)(n.p,{children:["The following settings ",(0,i.jsx)(n.em,{children:"are not"})," available in ",(0,i.jsx)(n.code,{children:"template.yml"}),", but can be defined as frontmatter at\nthe top of a template file. View the ",(0,i.jsx)(n.a,{href:"../guides/codegen#frontmatter",children:"code generation guide"})," for more\ninformation."]}),"\n",(0,i.jsx)(n.h3,{id:"force",children:(0,i.jsx)(n.code,{children:"force"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateFrontmatterConfig#force"}),"\n",(0,i.jsx)(n.p,{children:"When enabled, will always overwrite a file of the same name at the destination path, and will bypass\nany prompting in the terminal."}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-twig",children:"---\nforce: true\n---\n\nSome template content!\n"})}),"\n",(0,i.jsx)(n.h3,{id:"to",children:(0,i.jsx)(n.code,{children:"to"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateFrontmatterConfig#to"}),"\n",(0,i.jsx)(n.p,{children:"Defines a custom file path, relative from the destination root, in which to create the file. This\nwill override the file path within the template folder, and allow for conditional rendering and\nengine filters to be used."}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-twig",children:"{% set component_name = name | pascal_case %}\n\n---\nto: components/{{ component_name }}.tsx\n---\n\nexport function {{ component_name }}() {\n  return <div />;\n}\n"})}),"\n",(0,i.jsx)(n.h3,{id:"skip",children:(0,i.jsx)(n.code,{children:"skip"})}),"\n",(0,i.jsx)(l.Z,{to:"/api/types/interface/TemplateFrontmatterConfig#skip"}),"\n",(0,i.jsx)(n.p,{children:"When enabled, the template file will be skipped while writing to the destination path. This setting\ncan be used to conditionally render a file."}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-twig",children:'---\nskip: {{ name == "someCondition" }}\n---\n\nSome template content!\n'})})]})}function u(e={}){const{wrapper:n}={...(0,a.a)(),...e.components};return n?(0,i.jsx)(n,{...e,children:(0,i.jsx)(m,{...e})}):m(e)}},32189:(e,n,t)=>{t.d(n,{Z:()=>r});var i=t(83469),a=t(31792),l=t(24246);function r(e){let{to:n}=e;return(0,l.jsx)("a",{href:n,target:"_blank",className:"float-right inline-block",style:{marginTop:"-3em"},children:(0,l.jsx)(a.Z,{icon:i.dT$})})}},9785:(e,n,t)=>{t.d(n,{Z:()=>l});var i=t(9619),a=t(24246);function l(e){let{text:n="Required"}=e;return(0,a.jsx)(i.Z,{text:n,variant:"failure",className:"ml-2"})}},79022:(e,n,t)=>{t.d(n,{Z:()=>l});var i=t(9619),a=t(24246);function l(e){let{header:n,inline:t,updated:l,version:r}=e;return(0,a.jsx)(i.Z,{text:`v${r}`,variant:l?"success":"info",className:n?"absolute right-0 top-1.5":t?"inline-block":"ml-2"})}},9619:(e,n,t)=>{t.d(n,{Z:()=>s});var i=t(40624),a=t(31792),l=t(24246);const r={failure:"bg-red-100 text-red-900",info:"bg-pink-100 text-pink-900",success:"bg-green-100 text-green-900",warning:"bg-orange-100 text-orange-900"};function s(e){let{className:n,icon:t,text:s,variant:o}=e;return(0,l.jsxs)("span",{className:(0,i.Z)("inline-flex items-center px-1 py-0.5 rounded text-xs font-bold uppercase",o?r[o]:"bg-gray-100 text-gray-800",n),children:[t&&(0,l.jsx)(a.Z,{icon:t,className:"mr-1"}),s]})}},71670:(e,n,t)=>{t.d(n,{Z:()=>s,a:()=>r});var i=t(27378);const a={},l=i.createContext(a);function r(e){const n=i.useContext(l);return i.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function s(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(a):e.components||a:r(e.components),i.createElement(l.Provider,{value:n},e.children)}}}]);