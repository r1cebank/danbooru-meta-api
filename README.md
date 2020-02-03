# danbooru-meta-api
A node.js based microservice aimed to serve danbooru2019 dataset over API, with batch creation and training and verification data splitting.

## Usage

```
docker run -d -v metadbpath:/db/metadata.sqlite3 -p 3939:8000 r1cebank/danbooru-meta-api
```

## Endpoints

### GET /stat
Get the stat of the current metadata

**Response**
```json
{
    "num_posts": 3708763,
    "num_ratings": 3,
    "num_tags": 400147
}
```

### GET /tag/<id>
Get detailed info for a tag

**Response**
```json
{
    "category": 0,
    "id": 566835,
    "name": "multiple_girls"
}
```

### GET /posts/random?start=&end=&size
Get random posts, from start to end with size

**Response**

```json
{
    "count": 64,
    "result": [
        {
            "file_ext": "png",
            "file_size": 810555,
            "height": 1000,
            "id": 2431883,
            "location": "717/1574717.png",
            "md5": "bbd9f630aa2b72d909960633ccd94951",
            "pixiv_id": 40421351,
            "post_id": 1574717,
            "rating": "s",
            "source": "http://i1.pixiv.net/img11/img/hentay-shrimp/40421351_big_p0.png",
            "tags": [
                1821,
                13200,
                6059,
            ]
        }...
    ]
}
```

### GET /posts?batch_size=&batch_number=
Get regular batches, basic pagination

**Response**

```json
{
    "count": 64,
    "result": [
        {
            "file_ext": "png",
            "file_size": 810555,
            "height": 1000,
            "id": 2431883,
            "location": "717/1574717.png",
            "md5": "bbd9f630aa2b72d909960633ccd94951",
            "pixiv_id": 40421351,
            "post_id": 1574717,
            "rating": "s",
            "source": "http://i1.pixiv.net/img11/img/hentay-shrimp/40421351_big_p0.png",
            "tags": [
                1821,
                13200,
                6059,
            ]
        }...
    ]
}
```

### POST /posts/batch
Create a batch

**Eample Body**
```json
{
	"batch_size": 64, // Size of each batch
	"validation_split": 10, // Validation data percentage split
	"test_split": 10 // Test data percentage split
}
```
**Response**
```json
{
    "id": "df31e073520a47ffbeaec9d7ad74ebb2"
}
```

### GET /posts/batch/<id>/train/<batch_number>
Get a specific training batch for batch id

**Response**

```json
{
    "count": 64,
    "result": [
        {
            "file_ext": "png",
            "file_size": 810555,
            "height": 1000,
            "id": 2431883,
            "location": "717/1574717.png",
            "md5": "bbd9f630aa2b72d909960633ccd94951",
            "pixiv_id": 40421351,
            "post_id": 1574717,
            "rating": "s",
            "source": "http://i1.pixiv.net/img11/img/hentay-shrimp/40421351_big_p0.png",
            "tags": [
                1821,
                13200,
                6059,
            ]
        }...
    ]
}
```

### GET /posts/batch/<id>/test/<batch_number>
Get a specific test batch for batch id

**Response**

```json
{
    "count": 64,
    "result": [
        {
            "file_ext": "png",
            "file_size": 810555,
            "height": 1000,
            "id": 2431883,
            "location": "717/1574717.png",
            "md5": "bbd9f630aa2b72d909960633ccd94951",
            "pixiv_id": 40421351,
            "post_id": 1574717,
            "rating": "s",
            "source": "http://i1.pixiv.net/img11/img/hentay-shrimp/40421351_big_p0.png",
            "tags": [
                1821,
                13200,
                6059,
            ]
        }...
    ]
}
```

### GET /posts/batch/<id>/validation/<batch_number>
Get a specific validation batch for batch id

**Response**

```json
{
    "count": 64,
    "result": [
        {
            "file_ext": "png",
            "file_size": 810555,
            "height": 1000,
            "id": 2431883,
            "location": "717/1574717.png",
            "md5": "bbd9f630aa2b72d909960633ccd94951",
            "pixiv_id": 40421351,
            "post_id": 1574717,
            "rating": "s",
            "source": "http://i1.pixiv.net/img11/img/hentay-shrimp/40421351_big_p0.png",
            "tags": [
                1821,
                13200,
                6059,
            ]
        }...
    ]
}
```

### GET /posts/batch/<id>/info
Get info about a specific batch id

**Response**

```json
{
    "test_batches": 2897,
    "total_batches": 28974,
    "train_batches": 23180,
    "validation_batches": 2897
}
```
## N+1 problem with SQLite
Because we are using sqlite for the data backend (mainly for the portability), we are using executing many query with each api call, some might call this inefficient but with SQLite it is very efficient. As shown here https://www.sqlite.org/np1queryprob.html
