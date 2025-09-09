First we do the bencoding parsing engine.
Then we can decode .torrent files (which contain the actual info we need for the download).
Notably the torrent contains an announce URL, which is the URL of a tracker.
A downloader needs to perform a GET request to the tracker with required params (see the spec)
The tracker responds with bencoded dictionaries. It commonly returns a compact representation of the peer list (cf. BEP 23).
Then the peer protocol kicks in (probably?)
