
# Build and push to cachix
build-cachix auth_token:
    #!/usr/bin/env bash
    export CACHIX_AUTH_TOKEN="{{ auth_token }}"
    nix build .#default \
        --accept-flake-config \
        --no-link --print-out-paths  \
      | cachix push ultraflame4
