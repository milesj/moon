"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[74824],{69713:(e,o,n)=>{n.r(o),n.d(o,{assets:()=>d,contentTitle:()=>r,default:()=>p,frontMatter:()=>t,metadata:()=>c,toc:()=>a});var s=n(24246),i=n(71670),l=n(79022);const t={title:"Configuration",toc_max_heading_level:6},r=void 0,c={id:"proto/config",title:"Configuration",description:"We support configuration at both the project-level and user-level using a",source:"@site/docs/proto/config.mdx",sourceDirName:"proto",slug:"/proto/config",permalink:"/docs/proto/config",draft:!1,unlisted:!1,editUrl:"https://github.com/moonrepo/moon/tree/master/website/docs/proto/config.mdx",tags:[],version:"current",frontMatter:{title:"Configuration",toc_max_heading_level:6},sidebar:"proto",previous:{title:"Workflows",permalink:"/docs/proto/workflows"},next:{title:"Version specification",permalink:"/docs/proto/version-spec"}},d={},a=[{value:"Locations<VersionLabel></VersionLabel>",id:"locations",level:2},{value:"Where to configure?",id:"where-to-configure",level:3},{value:"Resolution mode<VersionLabel></VersionLabel>",id:"resolution-mode",level:2},{value:"<code>global</code>",id:"global",level:3},{value:"<code>local</code>",id:"local",level:3},{value:"<code>upwards</code>",id:"upwards",level:3},{value:"<code>upwards-global</code> / <code>all</code>",id:"upwards-global--all",level:3},{value:"Environment mode<VersionLabel></VersionLabel>",id:"environment-mode",level:2},{value:"Pinning versions",id:"pinning-versions",level:2},{value:"Lock <code>proto</code> version<VersionLabel></VersionLabel>",id:"lock-proto-version",level:3},{value:"Available settings",id:"available-settings",level:2},{value:"<code>[env]</code><VersionLabel></VersionLabel>",id:"env",level:3},{value:"<code>file</code><VersionLabel></VersionLabel>",id:"file",level:4},{value:"<code>[settings]</code>",id:"settings",level:3},{value:"<code>auto-install</code>",id:"auto-install",level:4},{value:"<code>auto-clean</code>",id:"auto-clean",level:4},{value:"<code>builtin-plugins</code><VersionLabel></VersionLabel>",id:"builtin-plugins",level:4},{value:"<code>detect-strategy</code>",id:"detect-strategy",level:4},{value:"<code>pin-latest</code>",id:"pin-latest",level:4},{value:"<code>telemetry</code>",id:"telemetry",level:4},{value:"<code>[settings.http]</code>",id:"settingshttp",level:3},{value:"<code>allow-invalid-certs</code>",id:"allow-invalid-certs",level:4},{value:"<code>proxies</code>",id:"proxies",level:4},{value:"<code>secure-proxies</code><VersionLabel></VersionLabel>",id:"secure-proxies",level:4},{value:"<code>root-cert</code>",id:"root-cert",level:4},{value:"<code>[settings.offline]</code><VersionLabel></VersionLabel>",id:"settingsoffline",level:3},{value:"<code>custom-hosts</code>",id:"custom-hosts",level:4},{value:"<code>override-default-hosts</code>",id:"override-default-hosts",level:4},{value:"<code>timeout</code>",id:"timeout",level:4},{value:"<code>[plugins]</code>",id:"plugins",level:3},{value:"Tool specific settings",id:"tool-specific-settings",level:2},{value:"<code>[tools.*]</code>",id:"tools",level:3},{value:"<code>[tools.*.aliases]</code>",id:"toolsaliases",level:3},{value:"<code>[tools.*.env]</code><VersionLabel></VersionLabel>",id:"toolsenv",level:3},{value:"<code>file</code><VersionLabel></VersionLabel>",id:"file-1",level:4},{value:"GitHub Action",id:"github-action",level:2}];function h(e){const o={a:"a",blockquote:"blockquote",code:"code",em:"em",h2:"h2",h3:"h3",h4:"h4",li:"li",p:"p",pre:"pre",ul:"ul",...(0,i.a)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsxs)(o.p,{children:["We support configuration at both the project-level and user-level using a\n",(0,s.jsx)(o.a,{href:"https://toml.io/en/",children:"TOML"})," based ",(0,s.jsx)(o.code,{children:".prototools"})," file. This file can be used to pin versions of\ntools, provide tool specific configuration, enable new tools via plugins, define proto settings, and\nmore."]}),"\n",(0,s.jsxs)(o.h2,{id:"locations",children:["Locations",(0,s.jsx)(l.Z,{version:"0.41.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["proto supports 3 locations in which a ",(0,s.jsx)(o.code,{children:".prototools"})," file can exist. These locations are used\nthroughout the command line and proto's own settings."]}),"\n",(0,s.jsxs)(o.ul,{children:["\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"local"})," -> ",(0,s.jsx)(o.code,{children:"./.prototools"})," (current directory)"]}),"\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"global"})," -> ",(0,s.jsx)(o.code,{children:"~/.proto/.prototools"})]}),"\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"user"})," -> ",(0,s.jsx)(o.code,{children:"~/.prototools"})]}),"\n"]}),"\n",(0,s.jsxs)(o.blockquote,{children:["\n",(0,s.jsxs)(o.p,{children:["Local is a bit of a misnomer as a ",(0,s.jsx)(o.code,{children:".prototools"})," file can theoretically exist in any directory, but\nwhen reading/writing to a file, ",(0,s.jsx)(o.code,{children:"local"})," refers to the current working directory."]}),"\n"]}),"\n",(0,s.jsx)(o.h3,{id:"where-to-configure",children:"Where to configure?"}),"\n",(0,s.jsxs)(o.p,{children:["With so many locations to store proto configuration, the question of where to store certain\nconfigurations become blurred, especially when ",(0,s.jsx)(o.a,{href:"#resolution-mode",children:"resolution"})," comes into play. We\nsuggest the following locations:"]}),"\n",(0,s.jsxs)(o.ul,{children:["\n",(0,s.jsxs)(o.li,{children:["Default/fallback ",(0,s.jsx)(o.a,{href:"#pinning-versions",children:"versions"})," of tools -> ",(0,s.jsx)(o.code,{children:"global"})]}),"\n",(0,s.jsxs)(o.li,{children:["Project specific ",(0,s.jsx)(o.a,{href:"#pinning-versions",children:"versions"})," of tools -> ",(0,s.jsx)(o.code,{children:"local"})]}),"\n",(0,s.jsxs)(o.li,{children:["Project specific ",(0,s.jsx)(o.a,{href:"#settings",children:"settings"})," -> ",(0,s.jsx)(o.code,{children:"local"})]}),"\n",(0,s.jsxs)(o.li,{children:["Shared/developer ",(0,s.jsx)(o.a,{href:"#settings",children:"settings"})," -> ",(0,s.jsx)(o.code,{children:"user"})]}),"\n",(0,s.jsxs)(o.li,{children:["Non-project related -> ",(0,s.jsx)(o.code,{children:"user"})]}),"\n"]}),"\n",(0,s.jsxs)(o.h2,{id:"resolution-mode",children:["Resolution mode",(0,s.jsx)(l.Z,{version:"0.40.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["When a ",(0,s.jsx)(o.code,{children:"proto"})," command or shim is ran, we must find and load all applicable ",(0,s.jsx)(o.code,{children:".prototools"})," files. We\nthen deeply merge all of these configuration files into a final configuration object, with the\ncurrent directory taking highest precedence."]}),"\n",(0,s.jsxs)(o.p,{children:["The order in which to resolve configuration can be defined using the ",(0,s.jsx)(o.code,{children:"--config-mode"})," (",(0,s.jsx)(o.code,{children:"-c"}),") command\nline option, or the ",(0,s.jsx)(o.code,{children:"PROTO_CONFIG_MODE"})," environment variable. The following 4 modes are supported:"]}),"\n",(0,s.jsx)(o.h3,{id:"global",children:(0,s.jsx)(o.code,{children:"global"})}),"\n",(0,s.jsxs)(o.p,{children:["In this mode, proto will ",(0,s.jsx)(o.em,{children:"only"})," load the ",(0,s.jsx)(o.code,{children:"~/.proto/.prototools"}),' file. This "global" file acts as\nconfiguration at the user-level and allows for fallback settings.']}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-text",children:"~/.proto/.prototools\n"})}),"\n",(0,s.jsx)(o.h3,{id:"local",children:(0,s.jsx)(o.code,{children:"local"})}),"\n",(0,s.jsxs)(o.p,{children:["In this mode, proto will ",(0,s.jsx)(o.em,{children:"only"})," load the ",(0,s.jsx)(o.code,{children:".prototools"})," file in the current directory."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-text",children:"./.prototools\n"})}),"\n",(0,s.jsx)(o.h3,{id:"upwards",children:(0,s.jsx)(o.code,{children:"upwards"})}),"\n",(0,s.jsxs)(o.p,{children:["In this mode, proto will traverse upwards starting from the current directory, and load\n",(0,s.jsx)(o.code,{children:".prototools"})," within each directory, until we reach the system root or the user directory (",(0,s.jsx)(o.code,{children:"~"}),"),\nwhichever comes first."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-text",children:"~/Projects/app/.prototools (cwd)\n~/Projects/.prototools\n~/.prototools\n"})}),"\n",(0,s.jsxs)(o.blockquote,{children:["\n",(0,s.jsxs)(o.p,{children:["This is the default mode for the ",(0,s.jsx)(o.a,{href:"./commands/activate",children:(0,s.jsx)(o.code,{children:"activate"})}),",\n",(0,s.jsx)(o.a,{href:"./commands/install",children:(0,s.jsx)(o.code,{children:"install"})}),", ",(0,s.jsx)(o.a,{href:"./commands/outdated",children:(0,s.jsx)(o.code,{children:"outdated"})}),", and\n",(0,s.jsx)(o.a,{href:"./commands/status",children:(0,s.jsx)(o.code,{children:"status"})})," commands."]}),"\n"]}),"\n",(0,s.jsxs)(o.h3,{id:"upwards-global--all",children:[(0,s.jsx)(o.code,{children:"upwards-global"})," / ",(0,s.jsx)(o.code,{children:"all"})]}),"\n",(0,s.jsxs)(o.p,{children:["This mode works exactly like ",(0,s.jsx)(o.a,{href:"#upwards",children:(0,s.jsx)(o.code,{children:"upwards"})})," but with the functionality of ",(0,s.jsx)(o.a,{href:"#global",children:(0,s.jsx)(o.code,{children:"global"})}),"\nas well. The global ",(0,s.jsx)(o.code,{children:"~/.proto/.prototools"})," file is appended as the final entry."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-text",children:"~/Projects/app/.prototools (cwd)\n~/Projects/.prototools\n~/.prototools\n~/.proto/.prototools\n"})}),"\n",(0,s.jsxs)(o.blockquote,{children:["\n",(0,s.jsxs)(o.p,{children:["This is the default mode for all other commands not listed above in ",(0,s.jsx)(o.code,{children:"upwards"}),"."]}),"\n"]}),"\n",(0,s.jsxs)(o.h2,{id:"environment-mode",children:["Environment mode",(0,s.jsx)(l.Z,{version:"0.29.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["We also support environment specific configuration, such as ",(0,s.jsx)(o.code,{children:".prototools.production"})," or\n",(0,s.jsx)(o.code,{children:".prototools.development"}),", when the ",(0,s.jsx)(o.code,{children:"PROTO_ENV"})," environment variable is set. This is useful for\ndefining environment specific aliases, or tool specific configuration."]}),"\n",(0,s.jsxs)(o.p,{children:["These environment aware settings take precedence over the default ",(0,s.jsx)(o.code,{children:".prototools"})," file, for the\ndirectory it's located in, and are merged in the same way as the default configuration. For example,\nthe lookup order would be the following when ",(0,s.jsx)(o.code,{children:"PROTO_ENV=production"}),":"]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-text",children:"~/Projects/.prototools.production\n~/Projects/.prototools\n~/.prototools.production\n~/.prototools\n~/.proto/.prototools\n"})}),"\n",(0,s.jsxs)(o.blockquote,{children:["\n",(0,s.jsxs)(o.p,{children:["The global ",(0,s.jsx)(o.code,{children:"~/.proto/.prototools"})," file does not support environment modes."]}),"\n"]}),"\n",(0,s.jsx)(o.h2,{id:"pinning-versions",children:"Pinning versions"}),"\n",(0,s.jsxs)(o.p,{children:["proto supports pinning versions of tools on a per-directory basis through our ",(0,s.jsx)(o.code,{children:".prototools"}),"\nconfiguration file. This file takes precedence during ",(0,s.jsx)(o.a,{href:"./detection",children:"version detection"})," and can be\ncreated/updated with ",(0,s.jsx)(o.a,{href:"./commands/pin",children:(0,s.jsx)(o.code,{children:"proto pin"})}),"."]}),"\n",(0,s.jsxs)(o.p,{children:["At its most basic level, you can map tools to specific versions, for the directory the file is\nlocated in. A ",(0,s.jsx)(o.a,{href:"./version-spec",children:"version"})," can either be a fully-qualified version, a partial version,\na range or requirement, or an alias."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'node = "16.16.0"\nnpm = "9"\ngo = "~1.20"\nrust = "stable"\n'})}),"\n",(0,s.jsxs)(o.h3,{id:"lock-proto-version",children:["Lock ",(0,s.jsx)(o.code,{children:"proto"})," version",(0,s.jsx)(l.Z,{version:"0.39.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["You can also pin the version of proto that you want all tools to execute with by adding a ",(0,s.jsx)(o.code,{children:"proto"}),"\nversion entry. This entry ",(0,s.jsx)(o.em,{children:"does not"})," support partial versions and ",(0,s.jsx)(o.em,{children:"must"})," contain a fully-qualified\nsemantic version."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'proto = "0.38.0"\n'})}),"\n",(0,s.jsxs)(o.p,{children:["Locking is currently enforced through ",(0,s.jsx)(o.a,{href:"./commands/activate",children:(0,s.jsx)(o.code,{children:"proto activate"})}),", and when activated,\nall shims (tools) will be executed with that explicit version of proto. This ",(0,s.jsx)(o.em,{children:"does not"})," apply to the\nversion of proto globally installed (in some situations), or for binaries linked in ",(0,s.jsx)(o.code,{children:"~/.proto/bin"}),"."]}),"\n",(0,s.jsx)(o.h2,{id:"available-settings",children:"Available settings"}),"\n",(0,s.jsxs)(o.h3,{id:"env",children:[(0,s.jsx)(o.code,{children:"[env]"}),(0,s.jsx)(l.Z,{version:"0.29.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["This setting is a map of environment variables that will be applied to ",(0,s.jsx)(o.em,{children:"all"})," tools when they are\nexecuted, or when ",(0,s.jsx)(o.a,{href:"./commands/activate",children:(0,s.jsx)(o.code,{children:"proto activate"})})," is ran in a shell profile. Variables\ndefined here ",(0,s.jsx)(o.em,{children:"will not"})," override existing environment variables (either passed on the command line,\nor inherited from the shell)."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[env]\nDEBUG = "*"\n'})}),"\n",(0,s.jsxs)(o.p,{children:["Additionally, ",(0,s.jsx)(o.code,{children:"false"})," can be provided as a value, which will ",(0,s.jsx)(o.em,{children:"remove"})," the environment variable. This\nis useful for removing inherited shell variables."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:"[env]\nDEBUG = false\n"})}),"\n",(0,s.jsxs)(o.p,{children:["Variables also support substitution using the syntax ",(0,s.jsx)(o.code,{children:"${VAR_NAME}"}),". When using substitution,\nvariables in the current process and merged ",(0,s.jsx)(o.code,{children:"[env]"})," can be referenced. Recursive substitution is not\nsupported!"]}),"\n",(0,s.jsxs)(o.blockquote,{children:["\n",(0,s.jsx)(o.p,{children:"This functionality enables per-directory environment variables!"}),"\n"]}),"\n",(0,s.jsxs)(o.h4,{id:"file",children:[(0,s.jsx)(o.code,{children:"file"}),(0,s.jsx)(l.Z,{version:"0.43.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["This is a special field that points to a dotenv file, relative from the current configuration file,\nthat will be loaded into the environment variables mapping. Variables defined in a dotenv file will\nbe loaded ",(0,s.jsx)(o.em,{children:"before"})," variables manually defined within ",(0,s.jsx)(o.code,{children:"[env]"}),"."]}),"\n",(0,s.jsxs)(o.p,{children:["This feature utilizes the ",(0,s.jsx)(o.a,{href:"https://github.com/allan2/dotenvy",children:"dotenvy"})," crate for parsing dotfiles."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[env]\nfile = ".env"\n'})}),"\n",(0,s.jsx)(o.h3,{id:"settings",children:(0,s.jsx)(o.code,{children:"[settings]"})}),"\n",(0,s.jsx)(o.h4,{id:"auto-install",children:(0,s.jsx)(o.code,{children:"auto-install"})}),"\n",(0,s.jsxs)(o.p,{children:["When enabled, will automatically installing missing tools when ",(0,s.jsx)(o.a,{href:"./commands/run",children:(0,s.jsx)(o.code,{children:"proto run"})})," is ran,\ninstead of erroring. Defaults to ",(0,s.jsx)(o.code,{children:"false"})," or ",(0,s.jsx)(o.code,{children:"PROTO_AUTO_INSTALL"}),"."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:"[settings]\nauto-install = true\n"})}),"\n",(0,s.jsx)(o.h4,{id:"auto-clean",children:(0,s.jsx)(o.code,{children:"auto-clean"})}),"\n",(0,s.jsxs)(o.p,{children:["When enabled, will automatically clean up the proto store in the background, by removing unused\ntools and outdated plugins. Defaults to ",(0,s.jsx)(o.code,{children:"false"})," or ",(0,s.jsx)(o.code,{children:"PROTO_AUTO_CLEAN"}),"."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:"[settings]\nauto-clean = true\n"})}),"\n",(0,s.jsxs)(o.h4,{id:"builtin-plugins",children:[(0,s.jsx)(o.code,{children:"builtin-plugins"}),(0,s.jsx)(l.Z,{version:"0.39.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["Can be used to customize the ",(0,s.jsx)(o.a,{href:"./tools#built-in",children:"built-in plugins"})," within proto. Can disable all\nbuilt-ins by passing ",(0,s.jsx)(o.code,{children:"false"}),", or enabling a select few by name. Defaults to ",(0,s.jsx)(o.code,{children:"true"}),", which enables\nall."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[settings]\n# Disable all\nbuiltin-plugins = false\n# Enable some\nbuiltin-plugins = ["node", "bun"]\n'})}),"\n",(0,s.jsx)(o.h4,{id:"detect-strategy",children:(0,s.jsx)(o.code,{children:"detect-strategy"})}),"\n",(0,s.jsxs)(o.p,{children:["The strategy to use when ",(0,s.jsx)(o.a,{href:"./detection",children:"detecting versions"}),". Defaults to ",(0,s.jsx)(o.code,{children:"first-available"})," or\n",(0,s.jsx)(o.code,{children:"PROTO_DETECT_STRATEGY"}),"."]}),"\n",(0,s.jsxs)(o.ul,{children:["\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"first-available"})," - Will use the first available version that is found. Either from ",(0,s.jsx)(o.code,{children:".prototools"}),"\nor a tool specific file (",(0,s.jsx)(o.code,{children:".nvmrc"}),", etc)."]}),"\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"prefer-prototools"})," - Prefer a ",(0,s.jsx)(o.code,{children:".prototools"})," version, even if found in a parent directory. If none\nfound, falls back to tool specific file."]}),"\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"only-prototools"})," - Only use a version defined in ",(0,s.jsx)(o.code,{children:".prototools"}),". ",(0,s.jsx)(l.Z,{version:"0.34.0"})]}),"\n"]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[settings]\ndetect-strategy = "prefer-prototools"\n'})}),"\n",(0,s.jsx)(o.h4,{id:"pin-latest",children:(0,s.jsx)(o.code,{children:"pin-latest"})}),"\n",(0,s.jsxs)(o.p,{children:['When defined and a tool is installed with the "latest" version, will automatically pin the resolved\nversion to the configured location. Defaults to disabled or ',(0,s.jsx)(o.code,{children:"PROTO_PIN_LATEST"}),"."]}),"\n",(0,s.jsxs)(o.ul,{children:["\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"global"})," - Pins globally to ",(0,s.jsx)(o.code,{children:"~/.proto/.prototools"}),"."]}),"\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"local"})," - Pins locally to ",(0,s.jsx)(o.code,{children:"./.prototools"})," in current directory."]}),"\n",(0,s.jsxs)(o.li,{children:[(0,s.jsx)(o.code,{children:"user"})," - Pins to the user's ",(0,s.jsx)(o.code,{children:"~/.prototools"})," in their home directory.","\n",(0,s.jsx)(l.Z,{version:"0.41.0"}),"\n"]}),"\n"]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[settings]\npin-latest = "local"\n'})}),"\n",(0,s.jsx)(o.h4,{id:"telemetry",children:(0,s.jsx)(o.code,{children:"telemetry"})}),"\n",(0,s.jsxs)(o.p,{children:["When enabled, we collect anonymous usage statistics for tool installs and uninstalls. This helps us\nprioritize which tools to support, what tools or their versions may be broken, the plugins currently\nin use, and more. Defaults to ",(0,s.jsx)(o.code,{children:"true"}),"."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:"[settings]\ntelemetry = false\n"})}),"\n",(0,s.jsxs)(o.blockquote,{children:["\n",(0,s.jsxs)(o.p,{children:["The data we track is publicly available and\n",(0,s.jsx)(o.a,{href:"https://github.com/moonrepo/proto/blob/master/legacy/cli/src/telemetry.rs",children:"can be found here"}),"."]}),"\n"]}),"\n",(0,s.jsx)(o.h3,{id:"settingshttp",children:(0,s.jsx)(o.code,{children:"[settings.http]"})}),"\n",(0,s.jsx)(o.p,{children:"Can be used to customize the HTTP client used by proto, primarily for requesting files to download,\navailable versions, and more."}),"\n",(0,s.jsx)(o.h4,{id:"allow-invalid-certs",children:(0,s.jsx)(o.code,{children:"allow-invalid-certs"})}),"\n",(0,s.jsxs)(o.p,{children:["When enabled, will allow invalid certificates instead of failing. This is an ",(0,s.jsx)(o.em,{children:"escape hatch"})," and\nshould only be used if other settings have failed. Be sure you know what you're doing! Defaults to\n",(0,s.jsx)(o.code,{children:"false"}),"."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:"[settings.http]\nallow-invalid-certs = true\n"})}),"\n",(0,s.jsx)(o.h4,{id:"proxies",children:(0,s.jsx)(o.code,{children:"proxies"})}),"\n",(0,s.jsxs)(o.p,{children:["A list of proxy URLs to use for requests. As an alternative, the ",(0,s.jsx)(o.code,{children:"HTTP_PROXY"})," and ",(0,s.jsx)(o.code,{children:"HTTPS_PROXY"}),"\nenvironment variables can be set. URLs that start with ",(0,s.jsx)(o.code,{children:"http://"})," will be considered insecure, while\n",(0,s.jsx)(o.code,{children:"https://"})," will be secure."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[settings.http]\nproxies = ["https://internal.proxy", "https://corp.net/proxy"]\n'})}),"\n",(0,s.jsxs)(o.h4,{id:"secure-proxies",children:[(0,s.jsx)(o.code,{children:"secure-proxies"}),(0,s.jsx)(l.Z,{version:"0.40.3"})]}),"\n",(0,s.jsx)(o.p,{children:"A list of proxy URLs that will be considered secure, regardless of the HTTP protocol."}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[settings.http]\nsecure-proxies = ["http://internal.proxy", "http://corp.net/proxy"]\n'})}),"\n",(0,s.jsx)(o.h4,{id:"root-cert",children:(0,s.jsx)(o.code,{children:"root-cert"})}),"\n",(0,s.jsxs)(o.p,{children:["The path to a root certificate to use for requests. This is useful for overriding the native\ncertificate, or for using a self-signed certificate, especially when in a corporate/internal\nenvironment. Supports ",(0,s.jsx)(o.code,{children:"pem"})," and ",(0,s.jsx)(o.code,{children:"der"})," files."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[settings.http]\nroot-cert = "/path/to/root/cert.pem"\n'})}),"\n",(0,s.jsxs)(o.h3,{id:"settingsoffline",children:[(0,s.jsx)(o.code,{children:"[settings.offline]"}),(0,s.jsx)(l.Z,{version:"0.41.0"})]}),"\n",(0,s.jsx)(o.p,{children:"Can be used to customize how we detect an internet connection for offline based logic. These\nsettings are useful if you're behind a VPN or corporate proxy."}),"\n",(0,s.jsx)(o.h4,{id:"custom-hosts",children:(0,s.jsx)(o.code,{children:"custom-hosts"})}),"\n",(0,s.jsxs)(o.p,{children:["A list of custom hosts to ping. Will be appended to our\n",(0,s.jsx)(o.a,{href:"#override-default-hosts",children:"default list of hosts"})," and will be ran last."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[settings.offline]\ncustom-hosts = ["proxy.corp.domain.com:80"]\n'})}),"\n",(0,s.jsx)(o.h4,{id:"override-default-hosts",children:(0,s.jsx)(o.code,{children:"override-default-hosts"})}),"\n",(0,s.jsx)(o.p,{children:"If our default hosts are blocked or are too slow, you can disable pinging them by setting this\noption to true. Our default hosts are Google DNS, Cloudflare DNS, and then Google and Mozilla hosts."}),"\n",(0,s.jsxs)(o.p,{children:["This should be used in parallel with ",(0,s.jsx)(o.a,{href:"#custom-hosts",children:(0,s.jsx)(o.code,{children:"custom-hosts"})}),"."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:"[settings.offline]\noverride-default-hosts = true\n"})}),"\n",(0,s.jsx)(o.h4,{id:"timeout",children:(0,s.jsx)(o.code,{children:"timeout"})}),"\n",(0,s.jsx)(o.p,{children:"The timeout in milliseconds to wait for a ping against a host to resolve. Default timeout is 750ms."}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:"[settings.offline]\ntimeout = 500\n"})}),"\n",(0,s.jsx)(o.h3,{id:"plugins",children:(0,s.jsx)(o.code,{children:"[plugins]"})}),"\n",(0,s.jsxs)(o.p,{children:["Additional ",(0,s.jsx)(o.a,{href:"./plugins",children:"plugins"})," can be configured with the ",(0,s.jsx)(o.code,{children:"[plugins]"})," section.\n",(0,s.jsx)(o.a,{href:"./plugins#enabling-plugins",children:"Learn more about this syntax"}),"."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[plugins]\nmy-tool = "https://raw.githubusercontent.com/my/tool/master/proto-plugin.toml"\n'})}),"\n",(0,s.jsx)(o.p,{children:"Once configured, you can run a plugin as if it was a built-in tool:"}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-shell",children:"$ proto install my-tool\n"})}),"\n",(0,s.jsx)(o.h2,{id:"tool-specific-settings",children:"Tool specific settings"}),"\n",(0,s.jsx)(o.h3,{id:"tools",children:(0,s.jsx)(o.code,{children:"[tools.*]"})}),"\n",(0,s.jsxs)(o.p,{children:["Tools support custom configuration that will be passed to their WASM plugin, which can be used to\ncontrol the business logic within the plugin. Please refer to the ",(0,s.jsx)(o.a,{href:"./tools",children:"official documentation"}),"\nof each tool (typically on their repository) for a list of available settings."]}),"\n",(0,s.jsxs)(o.p,{children:["As an example, let's configure ",(0,s.jsx)(o.a,{href:"https://github.com/moonrepo/node-plugin",children:"Node.js"})," (using the ",(0,s.jsx)(o.code,{children:"node"}),"\nidentifier)."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'npm = "bundled" # use bundled npm instead of specific version\n\n[tools.node]\nbundled-npm = true\n\n[tools.npm]\nshared-globals-dir = true\n'})}),"\n",(0,s.jsx)(o.h3,{id:"toolsaliases",children:(0,s.jsx)(o.code,{children:"[tools.*.aliases]"})}),"\n",(0,s.jsxs)(o.p,{children:["Aliases are custom and unique labels that map to a specific version, and can be configured manually\nwithin ",(0,s.jsx)(o.code,{children:".prototools"}),", or by calling the ",(0,s.jsx)(o.a,{href:"./commands/alias",children:(0,s.jsx)(o.code,{children:"proto alias"})})," command."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[tools.node.aliases]\nwork = "18"\noss = "20"\n'})}),"\n",(0,s.jsxs)(o.h3,{id:"toolsenv",children:[(0,s.jsx)(o.code,{children:"[tools.*.env]"}),(0,s.jsx)(l.Z,{version:"0.29.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["This setting is a map of environment variables for a specific tool, and will be applied when that\ntool is executed, or when ",(0,s.jsx)(o.a,{href:"./commands/activate",children:(0,s.jsx)(o.code,{children:"proto activate"})})," is ran in a shell profile. These\nvariables will override those defined in ",(0,s.jsx)(o.code,{children:"[env]"}),". Refer to ",(0,s.jsx)(o.a,{href:"#env",children:(0,s.jsx)(o.code,{children:"[env]"})})," for usage examples."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[tools.node.env]\nNODE_ENV = "production"\n'})}),"\n",(0,s.jsxs)(o.h4,{id:"file-1",children:[(0,s.jsx)(o.code,{children:"file"}),(0,s.jsx)(l.Z,{version:"0.43.0"})]}),"\n",(0,s.jsxs)(o.p,{children:["Like ",(0,s.jsx)(o.a,{href:"#file",children:(0,s.jsx)(o.code,{children:"[env].file"})}),", this is a path to a dotenv file, relative from the current configuration\nfile, that will be loaded into the environment variables mapping for this specific tool."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-toml",metastring:'title=".prototools"',children:'[tools.node.env]\nfile = "frontend/.env"\n'})}),"\n",(0,s.jsx)(o.h2,{id:"github-action",children:"GitHub Action"}),"\n",(0,s.jsxs)(o.p,{children:["To streamline GitHub CI workflows, we provide the\n",(0,s.jsx)(o.a,{href:"https://github.com/moonrepo/setup-toolchain",children:(0,s.jsx)(o.code,{children:"moonrepo/setup-toolchain"})})," action, which can be used\nto install ",(0,s.jsx)(o.code,{children:"proto"})," globally, and cache the toolchain found at ",(0,s.jsx)(o.code,{children:"~/.proto"}),"."]}),"\n",(0,s.jsx)(o.pre,{children:(0,s.jsx)(o.code,{className:"language-yaml",metastring:'title=".github/workflows/ci.yml"',children:"# ...\njobs:\n  ci:\n    name: 'CI'\n    runs-on: 'ubuntu-latest'\n    steps:\n      - uses: 'actions/checkout@v4'\n      - uses: 'moonrepo/setup-toolchain@v0'\n        with:\n          auto-install: true\n"})})]})}function p(e={}){const{wrapper:o}={...(0,i.a)(),...e.components};return o?(0,s.jsx)(o,{...e,children:(0,s.jsx)(h,{...e})}):h(e)}},79022:(e,o,n)=>{n.d(o,{Z:()=>l});var s=n(9619),i=n(24246);function l(e){let{header:o,inline:n,updated:l,version:t}=e;return(0,i.jsx)(s.Z,{text:`v${t}`,variant:l?"success":"info",className:o?"absolute right-0 top-1.5":n?"inline-block":"ml-2"})}},9619:(e,o,n)=>{n.d(o,{Z:()=>r});var s=n(40624),i=n(31792),l=n(24246);const t={failure:"bg-red-100 text-red-900",info:"bg-pink-100 text-pink-900",success:"bg-green-100 text-green-900",warning:"bg-orange-100 text-orange-900"};function r(e){let{className:o,icon:n,text:r,variant:c}=e;return(0,l.jsxs)("span",{className:(0,s.Z)("inline-flex items-center px-1 py-0.5 rounded text-xs font-bold uppercase",c?t[c]:"bg-gray-100 text-gray-800",o),children:[n&&(0,l.jsx)(i.Z,{icon:n,className:"mr-1"}),r]})}},71670:(e,o,n)=>{n.d(o,{Z:()=>r,a:()=>t});var s=n(27378);const i={},l=s.createContext(i);function t(e){const o=s.useContext(l);return s.useMemo((function(){return"function"==typeof e?e(o):{...o,...e}}),[o,e])}function r(e){let o;return o=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:t(e.components),s.createElement(l.Provider,{value:o},e.children)}}}]);