import asyncio
import pathlib
import sys

from pingchecker.checker import site_is_online, site_is_online_async
from pingchecker.cli import display_check_result, get_args


def main():
    """Run the Checker"""
    args = get_args()
    urls = _get_web_urls(args)
    if not urls:
        print("Error: no URLs to check", file=sys.stderr)
        sys.exit(1)
    if args.asynchronous:
        asyncio.run(_asynchronous_check(urls))
    else:
        _synchronous_check(urls)


def _get_web_urls(args):
    urls = args.urls
    if args.input_file:
        urls += _read_urls_from_file(args.input_file)
    return urls


def _read_urls_from_file(file):
    file_path = pathlib.Path(file)
    if file_path.is_file():
        with file_path.open(encoding="utf8") as urls_file:
            urls = [url.strip() for url in urls_file]
            if not urls:
                print(f"Error: empty input file {file}", file=sys.stderr)
            return urls
    else:
        print("Error: input file not found", file=sys.stderr)
    return []


async def _asynchronous_check(urls):
    async def _check(url):
        error = ""
        try:
            result = await site_is_online_async(url)
        except Exception as err:
            result = False
            error = str(err)
        display_check_result(result, url, error)

    await asyncio.gather(*(_check(url) for url in urls))


def _synchronous_check(urls):
    for url in urls:
        error = ""
        try:
            result = site_is_online(url)
        except Exception as err:
            result = False
            error = str(err)
        display_check_result(result, url, error)


if __name__ == "__main__":
    main()
