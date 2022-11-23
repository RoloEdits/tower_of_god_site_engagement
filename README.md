<h1 style="text-align: center;">ScrapeToon</h1>

A tool for scraping information from [Webtoons](https://www.webtoons.com/). Currently this is kept as generic as possible and focuses on the Daily Schedule page and any stories' home page.

## Functionality

### Daily Schedule

The data gathered from here is organized like so:

|title|author|genre|total_likes|status|
|:---:|:----:|:---:|:---------:|:----:|

The likes information, once it gets to the millions, is truncated, i.e. 1.1M. This data is nice for a broad cast with a net, but if you want more accuracy then you would need to use the other part of the scraper and scrape the stories page.

### Story Page

The data gathered from here is organized like so:

|title|author|genre|total_likes|status|release_day|views|subscribers|rating|chapter_number|likes|date|
|:---:|:----:|:---:|:---------:|:----:|:---------:|:---:|:---------:|:----:|:------------:|:---:|:--:|

The `chapter_number`, `likes`, and `date` are all relative to one chapter, with a new chapter on each row. The date is in the ISO 8601 format.

## Usage

Binary executables are provided for Windows, Mac, and Linux [here](https://github.com/RoloEdits/webtoon-scraper/releases).

The executable is stand alone and fully portable. Simply placing it in a folder is all that is needed to continue forward.

Once placed in a folder, simply right click in an empty part of the file explorer window and open the folder in a terminal.

From there you just need to enter `.\scrapetoon.exe` on Windows, or `./srapetoon` on Linux and Mac.

Once there you then have the option of which source of data you want to scrape: `daily` or `story`.

For example:

```bash
.\scrapetoon.exe daily
```

```bash
.\scrapetoon.exe story
```

`daily` requires only an output location to be given. This is done with either `--output` or the short version `-o` followed by the output directory.

For example:

```bash
.\scrapetoon.exe daily -o "D:\Desktop"
```

```bash
.\scrapetoon.exe daily --output "D:\Desktop"
```

`story` requires a bit more. Firstly a URL needs be given after, with the flags `-u | --url`.

```bash
.\scrapetoon.exe story --url "https://www.webtoons.com/en/action/omniscient-reader/list?title_no=2154"
```

And secondly, it also requires a numerical value to be given for an `end`. This value correlates to the page numbers below the chapter list. The scraper goes from 1 to the entered value. If you want all pages to be gone through, then you just enter the highest, the last, page.

<img src="imgs/omniscient_reader_page_numbers.png" style="display: block; margin-left: auto; margin-right: auto; width: 50%;: center">

In this case, if I want all pages, I enter 13

```bash
.\scrapetoon.exe story -u "https://www.webtoons.com/en/action/omniscient-reader/list?title_no=2154" -e 13
```

```bash
.\scrapetoon.exe story -url "https://www.webtoons.com/en/action/omniscient-reader/list?title_no=2154" --end-page 13
```

And same as before the `-o | --output` flag.

```bash
.\scrapetoon.exe story -u "https://www.webtoons.com/en/action/omniscient-reader/list?title_no=2154" -e 13 -o "D:\Desktop"
```

```bash
.\scrapetoon.exe story -url "https://www.webtoons.com/en/action/omniscient-reader/list?title_no=2154" --end-page 13 --output "D:\Desktop"
```

Once you have what you need entered in, press the `ENTER` key and it will begin its operation. If you entered an invalid output path, it will stop and inform you. From the entered path, a folder in created with the name of the current UTC date as its name. After that you will be prompted with a message of an attempt to connect, and once connected, a progress bar will render showing the elapsed time as well as the current amount done and what's needed.

The output files will either be `daily_schedule.csv` if you configured for `daily`, or `<STORY NAME>.csv` if you configured for `story`. In this examples case: `Omniscient Reader.csv`.

# License

Any CSV's hosted here are under a Creative Commons Zero v1.0 Universal License.

The software is under an MIT License.
