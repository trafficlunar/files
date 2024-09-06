# files
The code for my files server made in Rust hosted at https://files.axolotlmaid.com

## contents
- [features](#features)
- [how to use](#how-to-use)
- [installing](#installing)

## features
- Show contents / previews of files uploaded
- Highlighted uploaded code with highlight.js
- Uploading, deleting, and renaming with authorization
- View information of a specific file in JSON form
- Directory of uploads with optional authorization
- Generate password with specified length on start up for more security
- Option to generate filenames and its length
- Metrics for Prometheus and Grafana

## how to use
### uploading
To upload a file, make a POST request to the `/api/upload` route with the file attached alongside the file name and type and also with the header `Authorization` with a value of your password. 

### deleting
To delete a file, make a DELETE request to `/api/delete` with the `Authorization` header and set the body to the JSON example below:
```json
{
    "name": "file_name_here"
}
```

### renaming
To rename a file, do the same request as deleting but instead to the route `/api/rename` with the method PUT and set the body to the JSON example below:
```json
{
    "name": "file_name_here",
    "new_name": "new_file_name_here"
}
```

### view all uploads (directory)
To view all the uploads (also known as the directory), go to `/uploads` and enter the optional password. You can also delete and rename files and also upload on this page.

There is also an option to do this in JSON form, to do this, create a GET request to `/api/uploads` which will return every file uploaded in a JSON string array. The `Authorization` header may be required if `PROTECT_DIRECTORY` is enabled in .env.

### viewing files
There is two ways to view a specific file - you can either go to the URL with all the fancy styling which is `/uploads/file_name_here` or go to the same URL but with `/raw` and only the file itself at the end like `/uploads/file_name_here/raw`

### viewing information of a file
To view information of a file (name, size, modified time, url), create a GET request to `/uploads/filename/info` like you would do above but with `/info` as the suffix. This does not require authorization.

## installing
### docker (recommended)
1. Clone the repository
```bash
git clone https://github.com/axolotlmaid/files
cd files
```

2. (optional) Change the `favicon.ico` to whatever you want

3. Build the Docker image
```bash
docker build -t files .
```

4. Run the Docker image

> [!NOTE]
> Read [.env.example](https://github.com/axolotlmaid/files/blob/master/.env.example) for more information on these variables.

```bash
docker run -d \
    --restart=always \
    -e BASE_URL="http://localhost:3000" \
    -e PAGE_TITLE="files" \
    -e PASSWORD="" \
    -e PROTECT_DIRECTORY=true \
    -e ENABLE_FILE_ACTIONS_DIRECTORY=true \
    -e GENERATE_PASSWORD=true \
    -e GENERATE_PASSWORD_LENGTH=16 \
    -e GENERATE_FILENAME=false \
    -e GENERATE_FILENAME_LENGTH=8 \
    -e METRICS_ENABLED=true \
    -p 3000:3000 \
    -p 3001:3001 \
    --name files \
    files:latest
```

### manually
1. Clone the repository
```bash
git clone https://github.com/axolotlmaid/files
cd files
```

2. Rename `.env.preview` to `.env` and edit to your liking
3. (optional) Change the `favicon.ico` to whatever you want
4. Build and run the server
```bash
cargo build --release
./target/release/files
```