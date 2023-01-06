# umsatz-csv-to-sankeymatic

My first rust project.  
Tool to convert a bank csv transaction export to the format needed by https://sankeymatic.com/ to visualize your money flow.

## sankeymatic:
https://www.sankeymatic.com/build/?layout_style=auto&default_node_colorset=a&default_flow_inherit=outside_in&label_pos=inside&font_face=sans-serif
### example output image
![](https://i.redd.it/q0dfzzlxc3n41.png)

## troubleshooting
when your output image looks like this:  
![](examples/output_with_loop.png)
you have a circle in your ouput data

## csv example data:
[example file](data/transactions.csv)


## TODO list:
- clear input buffer before requesting input
- catch ctrl-c in enter labels to redo the regex
- way to skip csv lines in cli prompt
- fix circles
    - in streams capital first letter 
    - out streams small
- is csv header correct if creating new regex_labels.csv
- no regex -> whole line must match
    - escape line for use as regex! otherwise a transaction might match other transacions
      error with: `DE---iban---,12.04.22,12.04.22,ONLINE-UEBERWEISUNG,"stuff * + name DATUM 12.04.2022, 14.29 UHR ",,,,,,,adf.,DE---iban---,BIC,-130,EUR,Umsatz gebucht`
      `DE---iban---,28.02.22,28.02.22,ONLINE-UEBERWEISUNG,"Danke f�r ... :) hier ein Stack Geld DATUM 26.02.2022, 12.08 UHR ",,,,,,,name name,DE--iban--,BIC,-64,EUR,Umsatz gebucht`
- cli parameters
    - show all = see all entries as final labels
- cli ui
    - HARD: color labels while typing if they exist already to see if they're written correctrly
    - HARD: label autocomplete
    - HARD: regex see what matches?
- generate image automatically
    - is there an api for https://www.sankeymatic.com/build/?
    - download and render local with the sankeymatic js source?
- bugs
    - match without regex
    - wrong number displayed with this transaction: 
    `DE---iban---,07.10.22,07.10.22,ONLINE-UEBERWEISUNG,"Danke f�rs leien :D DATUM 06.10.2022, 20.59 UHR ",,,,,,,name name,DE---iban---,BIC,-50,EUR,Umsatz gebucht`
    `DE---iban---,28.02.22,28.02.22,ONLINE-UEBERWEISUNG,"Danke f�r ... :) hier ein Stack Geld DATUM 26.02.2022, 12.08 UHR ",,,,,,,name name,DE--iban--,BIC,-64,EUR,Umsatz gebucht`
    probably because encoding error ü?
- missing checks:
    - with sankeymatic.com you are not allowed to have circles.
      circle detection is missing 
      circles are quite easyly created with a regex, and a company refunding something!