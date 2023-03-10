<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, shrink-to-fit=no">
    <title>Technology World - VLE</title>
    <meta property="og:image" content="../../assets/img/og.svg">
    <meta name="description" content="Technology World provides beginner to advanced educational videos.">
    <meta property="og:type" content="website">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css">
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Open+Sans:400,500,600,700,800&amp;display=swap">
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto:400,500,700,900&amp;display=swap">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/animate.css/3.5.2/animate.min.css">
    <link rel="stylesheet" href="../../assets/css/styles.min.css">
</head>

<body style="background: var(--bs-body-bg);--bs-body-bg: #ffffff;">
    <nav class="navbar navbar-light navbar-expand-md sticky-top py-3" style="font-family: 'Open Sans', sans-serif;">
        <div class="container"><a class="navbar-brand d-flex align-items-center" href="#" style="font-family: Roboto, sans-serif;"><span class="fw-bold" id="logo" style="font-family: Roboto, sans-serif;">Technology World</span></a><button data-bs-toggle="collapse" class="navbar-toggler" data-bs-target="#navcol-1"><span class="visually-hidden">Toggle navigation</span><span class="navbar-toggler-icon"></span></button>
            <div class="collapse navbar-collapse" id="navcol-1">
                <ul class="navbar-nav me-auto">
                    <li class="nav-item"><a class="nav-link active" href="index.php">Home</a></li>
                    <li class="nav-item"><a class="nav-link active" href="pricing.php">Pricing</a></li>
                    <li class="nav-item"></li>
                    <li class="nav-item"></li>
                </ul>
            </div>
        </div>
    </nav>
    <section>
        <div>
            <div class="container py-4 py-xl-5">
                <?php
                $base_api_url = "https://technology-world.herokuapp.com";

                // Get course details from API
                $api_course = json_decode(file_get_contents("$base_api_url /api/v1/courses/122"), true)[0];

                // Get name and description from API
                $course_name = $api_course["course_name"];
                $course_description = $api_course["course_description"];

                // Print out the HTML of the course name and description
                echo<<<COURSE_TITLE
                    <div class="row mb-5">
                        <div class="col-md-8 col-xl-6 text-center mx-auto">
                            <h2>Welcome to, $course_name</h2>
                            <p>$course_description</p>
                        </div>
                    </div>
                    COURSE_TITLE;
                ?>
                <div class="row gy-4 row-cols-1 row-cols-md-2 row-cols-xl-3">
                    <?php
                        // Get video details from API
                        $api_video = json_decode(file_get_contents("$base_api_url /api/v1/videos/122"), true);

                        // Loop through each video
                        for ($i=0; $i < count($api_video); $i++) { 
                            $video_name = $api_video[$i]["video_title"];
                            $video_description = substr($api_video[$i]["video_description"], 0, 100);
                            $video_views = $api_video[$i]["video_views"];
                            $video_id = $api_video[$i]["video_id"];
                            $video_thumbnail = $api_video[$i]["video_thumbnail"];
                        
                            $channel_name = $api_video[$i]["channel_name"];
                            $channel_thumbnail = $api_video[$i]["channel_thumbnail"];    
                            
                            // Echo each video as a HTML element
                            echo<<<COURSE_CONTENT
                            <div class="col offset-xl-0">
                                <div class="card" data-bss-disabled-mobile="true" data-bss-hover-animate="pulse"><img class="card-img-top w-100 d-block fit-cover" style="height: 200px;" src="$video_thumbnail">
                                    <div class="card-body p-4">
                                        <h4 class="card-title">$video_name</h4>
                                        <p class="card-text">$video_description</p>
                                        <div class="d-flex"><img class="rounded-circle flex-shrink-0 me-3 fit-cover" width="50" height="50" src="$channel_thumbnail">
                                            <div>
                                                <p class="fw-bold mb-0">$channel_name</p>
                                                <p">$video_views views</p>
                                            </div>
                                        </div><a class="stretched-link" href="https://www.youtube.com/watch?v=$video_id"></a>
                                    </div>
                                </div>
                            </div>
                            COURSE_CONTENT;
                        }
                    ?>
                </div>
            </div>
        </div>
    </section>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/js/bootstrap.bundle.min.js"></script>
    <script src="assets/js/script.min.js"></script>
</body>
</html>