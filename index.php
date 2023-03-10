<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, shrink-to-fit=no">
    <title>Technology World - VLE</title>
    <meta property="og:image" content="assets/img/og.svg">
    <meta name="description" content="Technology World provides beginner to advanced educational videos.">
    <meta property="og:type" content="website">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css">
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Open+Sans:400,500,600,700,800&amp;display=swap">
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto:400,500,700,900&amp;display=swap">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/animate.css/3.5.2/animate.min.css">
    <link rel="stylesheet" href="assets/css/styles.min.css">
</head>

<body style="background: var(--bs-body-bg);--bs-body-bg: #ffffff;">
    <nav class="navbar navbar-light navbar-expand-md sticky-top py-3" style="font-family: 'Open Sans', sans-serif;">
        <div class="container"><a class="navbar-brand d-flex align-items-center" href="#" style="font-family: Roboto, sans-serif;"><span class="fw-bold" id="logo" style="font-family: Roboto, sans-serif;">Technology World</span></a><button data-bs-toggle="collapse" class="navbar-toggler" data-bs-target="#navcol-1"><span class="visually-hidden">Toggle navigation</span><span class="navbar-toggler-icon"></span></button>
            <div class="collapse navbar-collapse" id="navcol-1">
                <ul class="navbar-nav me-auto">
                    <li class="nav-item"><a class="nav-link active" href="#">Home</a></li>
                    <li class="nav-item"><a class="nav-link active" href="pricing.php">Pricing</a></li>
                    <li class="nav-item"></li>
                    <li class="nav-item"></li>
                </ul><a class="btn btn-primary me-2" href="login.php">Login</a><a class="text-bg-dark btn btn-primary me-2" href="register.php" style="background: rgb(13, 110, 253);">Register</a>
            </div>
        </div>
    </nav>
    <section>
        <div>
            <div class="container py-4 py-xl-5">
                <div class="row mb-5">
                    <div class="col-md-8 col-xl-6 text-center mx-auto">
                        <h2>Explore Courses</h2>
                        <p>Technology World has many courses <br>ranging from free to paid content</p>
                    </div>
                </div>
                <div class="row gy-4 row-cols-1 row-cols-md-2 row-cols-xl-3">
                <?php
                    $base_api_url = "https://technology-world.herokuapp.com";

                    // Get the courses from the API
                    $api_course = json_decode(file_get_contents("$base_api_url/api/v1/courses"), true);

                    // check if the API returned any courses
                    if ($api_course) {
                        // loop through the courses
                        for ($i=0; $i < count($api_course); $i++) { 
                            // Get the course, title, description, image
                            $course_title = $api_course[$i]["course_name"];
                            $course_description = $api_course[$i]["course_description"];
                            $course_image = $api_course[$i]["course_image"];
                            
                            // get course specific data
                            $data = file_get_contents("$base_api_url/api/v1/courses/".$api_course[$i]["course_id"]."/data");
                            // check if the API returned any data
                            if ($data) {
                                // get the channel name, thumbnail and total views
                                $api_channel = json_decode($data, true);
                                $course_leader_thumbnail = $api_channel["channel_thumbnail"];
                                $course_leader = $api_channel["channel_name"];
                                $course_total_views = $api_channel["course_total_views"];
                                $course_link = sprintf("/courses/%u/content.php", $api_course[$i]["course_id"]);
                                
                                // output the course
                                echo<<<_END
                                <div class="col offset-xl-0">
                                    <div class="card" data-bss-disabled-mobile="true" data-bss-hover-animate="pulse"><img class="card-img-top w-100 d-block fit-cover" style="height: 200px;" src="$course_image">
                                        <div class="card-body p-4">
                                            <h4 class="card-title">$course_title</h4>
                                            <p class="card-text">$course_description</p>
                                            <div class="d-flex"><img class="rounded-circle flex-shrink-0 me-3 fit-cover" width="50" height="50" src="$course_leader_thumbnail">
                                                <div>
                                                    <p class="fw-bold mb-0">$course_leader</p>
                                                    <p>$course_total_views views</p>
                                                </div>
                                            </div><a class="stretched-link" href="$course_link"></a>
                                        </div>
                                    </div>
                                </div>
                                _END;
                            }
                        }
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