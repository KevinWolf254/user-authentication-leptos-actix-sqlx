use leptos::*;

#[component]
pub fn ForgotPasswordPage() -> impl IntoView {
    view! {
        <section class="h-100">
            <div class="container h-100">
                <div class="row justify-content-sm-center h-100">
                    <div class="col-xxl-4 col-xl-5 col-lg-5 col-md-7 col-sm-9">
                        <div class="text-center my-5">
                            <img src="https://getbootstrap.com/docs/5.0/assets/brand/bootstrap-logo.svg" alt="logo" width="100"/>
                        </div>
                        <div class="card shadow-lg">
                            <div class="card-body p-5">
                                <h1 class="fs-4 card-title fw-bold mb-4">Forgot Password</h1>
                                <form method="POST" class="needs-validation" novalidate="" autocomplete="off">
                                    <div class="mb-3">
                                        <label class="mb-2 text-muted" for="email">Email Address</label>
                                        <input id="email" type="email" class="form-control" name="email" value="" required autofocus/>
                                        <div class="invalid-feedback">
                                            Email is invalid
                                        </div>
                                    </div>
    
                                    <div class="d-flex align-items-center">
                                        <button type="submit" class="btn btn-primary ms-auto">
                                            Send Link	
                                        </button>
                                    </div>
                                </form>
                            </div>
                            <div class="card-footer py-3 border-0">
                                <div class="text-center">
                                    Remember your password? <a href="index.html" class="text-dark">Login</a>
                                </div>
                            </div>
                        </div>
                        <div class="text-center mt-5 text-muted">
                            Copyright &copy; 2017-2021 &mdash; Your Company 
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}