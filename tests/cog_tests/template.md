{% if version.tag and from.tag -%}                                                                                               
## [{{ version.tag }}]({{repository_url ~ "/compare/" ~ from.tag ~ ".." ~ version.tag}}) - {{ date | date(format="%Y-%m-%d") }}
{% elif version.tag and from.id -%}                                                                                              
## [{{ version.tag }}]({{repository_url ~ "/compare/" ~ from.id ~ ".." ~ version.tag}}) - {{ date | date(format="%Y-%m-%d") }}
{% else -%}                                                                                                                      
                                                                                                                                
{% set from = from.id -%}                                                                                                        
{% set to = version.id -%}                                                                                                       
{% set from_shorthand = from.id | truncate(length=7, end="") -%}                                                                 
{% set to_shorthand = version.id | truncate(length=7, end="") -%}                                                                
## Unreleased ([{{ from_shorthand ~ ".." ~ to_shorthand }}]({{repository_url ~ "/compare/" ~ from_shorthand ~ ".." ~ to_shorthand}}))
{% endif -%}                                                                                                                     
{% for type, typed_commits in commits | sort(attribute="type")| group_by(attribute="type")-%}                                    
#### {{ type | upper_first }}
{% for commit in typed_commits -%}                                                                                           
    {% if commit.author and repository_url -%}                                                                                
    {% set author = "@" ~ commit.author -%}                                                                                   
    {% set author_link = platform ~ "/" ~ commit.author -%}                                                                   
    {% set author = "[" ~ author ~ "](" ~ author_link ~ ")" -%}                                                               
    {% else -%}                                                                                                               
    {% set author = commit.signature -%}                                                                                      
    {% endif -%}                                                                                                              
                                                                                                                                
    {% if commit.scope -%}                                                                                                    
    {% set scope = "**(" ~ commit.scope ~ ")**" -%}                                                                             
    {% else -%}                                                                                                                 
    {% set scope = "" -%}                                                                                                       
    {% endif -%}                                                                                                               
    
    {% set commit_link = repository_url ~ "/commit/" ~ commit.id -%}                                                           
    {% set shorthand = commit.id | truncate(length=7, end="") -%}                                                                     
    - {{ scope }} {{ commit.summary }} - ([{{shorthand}}]({{ commit_link }})) - {{ author }}
{% endfor -%}                                                                                                                

{% endfor -%}                                                                                                                    