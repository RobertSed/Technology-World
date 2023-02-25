<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, shrink-to-fit=no">
    <title>commie</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css">
    <link rel="stylesheet" href="assets/css/styles.min.css">
</head>

<body>
    <nav class="navbar navbar-light navbar-expand-md py-3" style="border-bottom-width: 1px;border-bottom-style: solid;">
        <div class="container"><a class="navbar-brand d-flex align-items-center" href="#"><span>Technology World VLE</span></a><button data-bs-toggle="collapse" class="navbar-toggler" data-bs-target="#navcol-2"><span class="visually-hidden">Toggle navigation</span><span class="navbar-toggler-icon"></span></button>
            <div class="collapse navbar-collapse" id="navcol-2">
                <ul class="navbar-nav ms-auto"></ul>
                <ul class="navbar-nav">
                    <li class="nav-item"><a class="nav-link active" href="#">First Item</a></li>
                    <li class="nav-item"><a class="nav-link" href="#">Second Item</a></li>
                    <li class="nav-item"><a class="nav-link" href="#">Nav Item</a></li>
                    <li class="nav-item"></li>
                </ul><button class="btn btn-primary" type="button">Login</button>
            </div>
        </div>
    </nav>
    <div class="container py-4 py-xl-5">
        <div class="row gy-4 row-cols-1 row-cols-md-2 row-cols-xl-3">
            <?php
                $video_thumbnail = "https://i.ytimg.com/vi/8Qn_spdM5Zg/maxresdefault.jpg";
                $video_title = "How to make a website";
                $video_descrption = "This is a video on how to make a website";
                for($x = 0; $x <= 5; $x++) {
                    Echo <<<_END
                        <div class="col">
                                <div><img class="rounded img-fluid d-block w-100 fit-cover" style="height: 200px;" src="$video_thumbnail" />
                                    <div class="py-4">
                                        <h4>$video_title</h4>
                                        <p>$video_descrption</p>
                                    </div>
                                </div>
                        </div>
                    _END;
                }
            ?>
        </div>
    </div>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/js/bootstrap.bundle.min.js"></script>
</body>

</html>