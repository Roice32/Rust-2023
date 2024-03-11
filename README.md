# wiki_stats

#### A tool that reads the zip archive provided, and extracts the below information from all the data in all the jsons. The output will be written to a file.
- a frequency list of all the words as written
- a frequency list of all the words as lowercase
- the title, the json path in the zip, and the size of the longest article
- the title, the json path in the zip, and the size of the longest title

####  The tool does the processing using multithreaded techniques, as to make the search as fast as possible.
    
## Tool info (as outputted by help command):
####  Description: Tool for analyzing article datasets stored as .JSON files within a .zip archive.
Uses multithreaded techniques to iterate through each file, calculating each word's number of appearences (as-written & lowercased), as well as info about the longest article & title.
Maximum number of parallel threads running = number of virtual threads on the CPU.
#### Available command-line arguments:
1. --aide -a: Displays this info about the program. French for "help" because "help" gets into conflict with cargo's own "--help";
2. --input -i [source_file.zip]: Specifies the file containing the dataset to be analyzed. Must be a .zip archive. Default: 'datasets\dataset.zip';
3. --output -o [output_file.txt]: Specifies the file where computed stats will be written. Must be a .txt file. Default: 'stats.txt';
4. --metrics -m: During execution will print:
  - partial progress: files processed / total files* + precentage (*all files counted, even if not .JSON)
  - total time elapsed during file processing: secs & milisecs, #files processed, their total compressed size
  - total time elapsed during output writing: secs & milisecs;
5. --plain -p: Computed stats will be written to file as plain text tuples, not formatted as JSONs (faster write time).

### Example run:
![Post-Execution](/project/wiki_stats/run2.png)
