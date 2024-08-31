# files
The code for my files server made in Rust hosted at https://files.axolotlmaid.com

## features
- Show contents / previews of files uploaded
- Uploading and deleting with authorization
- Generate password on start up for more security
- Metrics for Prometheus and Grafana

## how to use
### uploading
To upload a file, make a POST request to the `/upload` route with the header `Authorization` along with a value of `Bearer password_here`.

### deleting
To delete a file, do the same request as uploading but use the DELETE method and set the body to the example below.

```json
{
    "name": "file_name_here"
}
```

### view specific file
There is two ways to view a specific file - you can either go to the URL which is `/uploads/file_name_here` or go to the same URL but with `/raw` at the end like `/uploads/file_name_here/raw`

## installing
1. Clone the repository
```bash
git clone https://github.com/axolotlmaid/files
cd files
```

2. Rename `.env.preview` to `.env` and edit to your liking

3. (optional) Change the `favicon.ico` to whatever you want

4. Run the server