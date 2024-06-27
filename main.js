async function recognize(base64, _lang, options) {
    const { config, utils } = options;
    const { tauriFetch, cacheDir, readBinaryFile, http } = utils;
    let { session_id } = config;
    base64 = `data:image/png;base64,${base64}`;

    if (session_id === undefined || session_id.length === 0) {
        throw "session_id not found";
    }

    let file_path = `${cacheDir}pot_screenshot_cut.png`;
    let fileContent = await readBinaryFile(file_path);

    let res = await http.fetch('https://p2t.breezedeus.com/api/pix2text', {
        method: "POST",
        headers: {
            'content-type': 'multipart/form-data',
        },
        body: http.Body.form(
            {
                image: {
                    file: fileContent,
                    mime: 'image/png',
                    fileName: 'pot_screenshot_cut.png',
                },
                session_id: session_id,
            }
        )
    })

    if (res.ok) {
        const { message, results } = res.data;
        if (message !== "success") {
            throw JSON.stringify(res);
        }
        if (results) {
            return results;
        } else {
            throw JSON.stringify(res);
        }
    } else {
        throw JSON.stringify(res);
    }
}
