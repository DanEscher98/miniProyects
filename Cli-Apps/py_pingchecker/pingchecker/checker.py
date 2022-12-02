""" Check if a online site is active """

import asyncio
import socket
from http.client import HTTPConnection
from urllib.parse import urlparse

import aiohttp


def site_is_online(url, timeout=2):
    """If the target URL is online, return True.
    Otherwise, raise an exception."""
    error = Exception("unknown error")
    parser = urlparse(url)
    # extract the hostname from the target URL
    host = parser.netloc or parser.path.split("/")[0]
    for port in (80, 443):
        connection = HTTPConnection(host=host, port=port, timeout=timeout)
        try:
            connection.request("HEAD", "/")
            return True
        except socket.gaierror as err:
            error = err
        finally:
            connection.close()
    raise error


async def site_is_online_async(url, timeout=2) -> bool:
    """If the target URL is online, return True.
    Otherwise, raise  an exception."""
    error = Exception("unknown error")
    parser = urlparse(url)
    host = parser.netloc or parser.path.split("/")[0]
    for scheme in ("http", "https"):
        target_url = scheme + "://" + host
        async with aiohttp.ClientSession() as session:
            try:
                await session.head(target_url, timeout=timeout)
                return True
            except asyncio.exceptions.TimeoutError:
                error = Exception("Timeout")
            except socket.gaierror as err:
                error = err
    raise error
