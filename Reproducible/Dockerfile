# many network need proxy . host run docker cmd need proxy .
# inside the docker container need proxy. 
# need to set for this two env. 
# or. use US azure server 

####    1.set up env:
# Use ubuntu 2204 lts as the base image ,jammy is ubuntu 2204 lts 
FROM ubuntu:jammy
# Install node and npm
RUN apt-get update && apt-get install -y nodejs npm

# Set the node and npm versions
RUN npm install -g n && n 18.17.1 && npm install -g npm@9.6.7

# Install dfx
RUN sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
# Set the dfx version
RUN dfx upgrade 0.14.3
# RUN DFX_VERSION=0.14.3 sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"

# Install cargo and rustc
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set the cargo and rustc versions
RUN rustup install 1.72.0 && rustup default 1.72.0

# install target architect dependencies
RUN rustup target add wasm32-unknown-unknown wasm32-wasi x86_64-unknown-linux-gnu


####    2.build taxlint 
RUN mkdir ~/taxlint
WORKDIR ~/taxlint
RUN apt install git 
RUN git clone https://github.com/TaxLintDAO/taxlint.git
RUN npm install
RUN npm run build
RUN cargo build --target wasm32-unknown-unknown --release --package "backend"  --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm"  > ./backend/backend.did  --allow-precompiled 
RUN dfx start --background
RUN dfx deploy
RUN touch local_module_hashes.txt 
RUN dfx canister status --all | grep "Module hash" | awk '{ print $3 }' > local_module_hashes.txt
# compare hash :
RUN diff ./Reproducible/ic_module_hashes.txt ./local_module_hashes.txt


#### 3.build the image and run 
# docker build -t taxlint:v1 . \
# --build-arg "HTTP_PROXY=http://192.168.235.1:25526/" \
#     --build-arg "HTTPS_PROXY=http://192.168.235.1:25526/" \
# need proxy sometimes: 
# docker build --build-arg http_proxy="http://127.0.0.53:25526" --build-arg https_proxy="http://127.0.0.53:25526" -t taxlint:v1 .
# docker build --build-arg http_proxy="http://127.0.0.1:25526" --build-arg https_proxy="http://127.0.0.1:25526" -t taxlint:v1 .
# docker build --build-arg http_proxy="http://192.168.235.1:25526" --build-arg https_proxy="http://192.168.235.1:25526" -t taxlint:v1 .
# docker run -it taxlint:v1

#### 4.push image to docker-hub
# docker login ...
# docker tag taxlint:v1 btwl/taxlint:v1

#### 5.user use : 
# docker run  btwl/taxlint:v1