# danbooru-meta-api
A node.js based microservice aimed to serve danbooru2019 dataset over API, with batch creation and training and verification data splitting.

## Usage

## N+1 problem with SQLite
Because we are using sqlite for the data backend (mainly for the portability), we are using executing many query with each api call, some might call this inefficient but with SQLite it is very efficient. As shown here https://www.sqlite.org/np1queryprob.html
