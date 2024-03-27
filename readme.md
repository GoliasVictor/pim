# Command-Line Help for `pm`

This document contains the help content for the `pm` command-line program.

**Command Overview:**

* [`pm`↴](#pm)
* [`pm find`↴](#pm-find)
* [`pm open`↴](#pm-open)
* [`pm dir`↴](#pm-dir)
* [`pm list`↴](#pm-list)
* [`pm run`↴](#pm-run)
* [`pm new`↴](#pm-new)
* [`pm info`↴](#pm-info)
* [`pm info is-env`↴](#pm-info-is-env)
* [`pm info property`↴](#pm-info-property)
* [`pm completions`↴](#pm-completions)

## `pm`

Project Manager

**Usage:** `pm [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `find` — Find environments who match the filters
* `open` — Open a environmnet (run the `open` script or open vscode in the environment directory)
* `dir` — Show the directory of project
* `list` — List projects
* `run` — Run a script
* `new` — Create a new environment
* `info` — Get information about an environment
* `completions` — 

###### **Options:**

* `-r`, `--root <ROOT>` — The root folder with all environments to map and run commands



## `pm find`

Find environments who match the filters

**Usage:** `pm find [OPTIONS]`

###### **Options:**

* `-n`, `--name <NAME>` — Part of the environment name
* `-l`, `--language <LANGUAGE>` — Environment language
* `-c`, `--categories <CATEGORIES>` — Environment category
* `-t`, `--type <TYPE>` — Environment type

  Possible values:
  - `folder`:
    Folder: an environment that groups other environments
  - `project`:
    Project: an environment that runs and has source code and can be used alone
  - `sub-project`:
    Subproject: a part within a project, a module, a layer, a library, etc. an internal separation within a project

* `-m`, `--max-depth <MAX_DEPTH>` — Maximum depth to find



## `pm open`

Open a environmnet (run the `open` script or open vscode in the environment directory)

**Usage:** `pm open <ENVIRONMENT>`

###### **Arguments:**

* `<ENVIRONMENT>` — Environmnet to open



## `pm dir`

Show the directory of project

**Usage:** `pm dir <ENVIRONMENT>`

###### **Arguments:**

* `<ENVIRONMENT>` — Name of the environment from which the directory will be geted



## `pm list`

List projects

**Usage:** `pm list [OPTIONS] [FOLDER]`

###### **Arguments:**

* `<FOLDER>` — The folder to child environments

###### **Options:**

* `-s`, `--style <STYLE>` — The style of output

  Default value: `tree`

  Possible values:
  - `flat`:
    Print environment name line by line
  - `tree`:
    Print environment in tree
  - `by-language`:
    Print environment of each language
  - `by-category`:
    Print environment of each category

* `-t`, `--type <TYPE>` — Maximum environment type, subproject: print subproject, project and subproject, project: print subproject and project. folder: prints only folders

  Possible values:
  - `folder`:
    Folder: an environment that groups other environments
  - `project`:
    Project: an environment that runs and has source code and can be used alone
  - `sub-project`:
    Subproject: a part within a project, a module, a layer, a library, etc. an internal separation within a project

* `-m`, `--max-depth <MAX_DEPTH>` — Maximum folder depth to show



## `pm run`

Run a script

**Usage:** `pm run [OPTIONS] [SCRIPT] [-- <PARAMETERS>...]`

###### **Arguments:**

* `<SCRIPT>` — Name of script
* `<PARAMETERS>` — Parameters added to the end of the script

###### **Options:**

* `-e`, `--environmnet <ENVIRONMNET>` — Environmnet where is the script



## `pm new`

Create a new environment

**Usage:** `pm new [OPTIONS] [TEMPLATE]`

###### **Arguments:**

* `<TEMPLATE>` — template of environment

###### **Options:**

* `-p`, `--path <PATH>` — Path to environment



## `pm info`

Get information about an environment

**Usage:** `pm info [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `is-env` — Check if the path is within an environment
* `property` — get a property from the environment

###### **Options:**

* `-e`, `--environment <ENVIRONMENT>` — Environment where the information will be obtained (if this and path are not specified, use the environment close to the root from the current folder)
* `-p`, `--path <PATH>` — Path to the environment  (if `--environment` is specified ignore, this will be ignored)



## `pm info is-env`

Check if the path is within an environment

**Usage:** `pm info is-env`



## `pm info property`

get a property from the environment

**Usage:** `pm info property <PROPERTY>`

###### **Arguments:**

* `<PROPERTY>` — The property to get of the environment

  Possible values: `environment-type`, `languages`, `path`, `description`, `script-interpreter`, `name`, `directory`




## `pm completions`

**Usage:** `pm completions <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell to generate the completions for

  Possible values: `bash`, `elvish`, `fig`, `fish`, `nushell`, `powershell`, `zsh`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

