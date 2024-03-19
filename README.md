# get_link


## Overview
this program is CI tool. build with Rust.
you can get list of links in webpage


## Requirement
Rust,
Cargo

## Usage
get_link https://www.xxx.xx.com/
return list of <a> tag links.

##options

--filter / -f
you can use filter option.
for example
get_link https://xxx.xx.com --filter article

return list of links contains "artcle" string

--substring / -s
remove characters from url
ex:
get_link https://xxx.com -s 2

remove 2characters from head.
for all of list.


## Author
masamichi

## Licence

MIT