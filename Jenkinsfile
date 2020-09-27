G_giturl = "git@github.com:tonlabs/TON-SDK.git"
G_gitcred = 'TonJenSSH'
G_docker_creds = 'dockerhubLanin'
C_PROJECT = "NotSet"
C_COMMITER = "NotSet"
C_HASH = "NotSet"
C_TEXT = "NotSet"
G_binversion = "NotSet"
G_tsnj_build = true
G_tsnj_deploy = true
G_images = [:]
G_branches = [:]
G_params = null

def getVar(Gvar) {
    return Gvar
}

def isUpstream() {
    return currentBuild.getBuildCauses()[0]._class.toString() == 'hudson.model.Cause$UpstreamCause'
}

def buildImagesMap() {
    G_images.put('ton-types', params.image_ton_types)
    G_images.put('ton-labs-types', params.image_ton_labs_types)
    G_images.put('ton-block', params.image_ton_block)
    G_images.put('ton-labs-block', params.image_ton_labs_block)
    G_images.put('ton-vm', params.image_ton_vm)
    G_images.put('ton-labs-vm', params.image_ton_labs_vm)
    G_images.put('ton-labs-abi', params.image_ton_labs_abi)
    G_images.put('ton-executor', params.image_ton_executor)
    G_images.put('ton-labs-executor', params.image_ton_labs_executor)
    G_images.put('tvm-linker', params.image_tvm_linker)
    G_images.put('ton-sdk', "")
}

def buildBranchesMap() {
    if (params.branch_ton_types == '') {
        G_branches.put('ton-types', "master")
    } else {
        G_branches.put('ton-types', params.branch_ton_types)
    }
    
    if (params.branch_ton_labs_types == '') {
        G_branches.put('ton-labs-types', "release-candidate")
    } else {
        G_branches.put('ton-labs-types', params.branch_ton_labs_types)
    }

    if (params.branch_ton_block == '') {
        G_branches.put('ton-block', "master")
    } else {
        G_branches.put('ton-block', params.branch_ton_block)
    }

    if (params.branch_ton_labs_block == '') {
        G_branches.put('ton-labs-block', "release-candidate")
    } else {
        G_branches.put('ton-labs-block', params.branch_ton_labs_block)
    }

    if (params.branch_ton_vm == '') {
        G_branches.put('ton-vm', "master")
    } else {
        G_branches.put('ton-vm', params.branch_ton_vm)
    }

    if (params.branch_ton_labs_vm == '') {
        G_branches.put('ton-labs-vm', "release-candidate")
    } else {
        G_branches.put('ton-labs-vm', params.branch_ton_labs_vm)
    }

    if (params.branch_ton_labs_abi == '') {
        G_branches.put('ton-labs-abi', "master")
    } else {
        G_branches.put('ton-labs-abi', params.branch_ton_labs_abi)
    }

    if (params.branch_ton_executor == '') {
        G_branches.put('ton-executor', "master")
    } else {
        G_branches.put('ton-executor', params.branch_ton_executor)
    }

    if (params.branch_ton_labs_executor == '') {
        G_branches.put('ton-labs-executor', "master")
    } else {
        G_branches.put('ton-labs-executor', params.branch_ton_labs_executor)
    }

    if (params.branch_ton_sdk != "${env.BRANCH_NAME}") {
        G_branches.put('ton-sdk', "${env.BRANCH_NAME}")
    } else {
        G_branches.put('ton-sdk', params.branch_ton_sdk)
    }

    if (params.branch_tvm_linker == '') {
        G_branches.put('tvm-linker', "master")
    } else {
        G_branches.put('tvm-linker', params.branch_tvm_linker)
    }

    if (params.branch_sol2tvm == '') {
        G_branches.put('sol2tvm', "master")
    } else {
        G_branches.put('sol2tvm', params.branch_sol2tvm)
    }
}

def buildParams() {
    buildImagesMap()
    if(!isUpstream() && GIT_BRANCH != 'master' && !(GIT_BRANCH ==~ '^PR-[0-9]+')) {
        G_images['ton-types'] = 'tonlabs/ton-types:latest'
        G_images['ton-labs-types'] = 'tonlabs/ton-labs-types:latest'
        G_images['ton-block'] = 'tonlabs/ton-block:latest'
        G_images['ton-labs-block'] = 'tonlabs/ton-labs-block:latest'
        G_images['ton-vm'] = 'tonlabs/ton-vm:latest'
        G_images['ton-labs-vm'] = 'tonlabs/ton-labs-vm:latest'
        G_images['ton-executor'] = 'tonlabs/ton-executor:latest'
        G_images['ton-labs-executor'] = 'tonlabs/ton-labs-executor:latest'
        G_images['ton-labs-abi'] = 'tonlabs/ton-labs-abi:latest'
    }
    buildBranchesMap()
    G_params = []
    params.each { key, value ->
        def item = null
        def nKey = key.toLowerCase().replaceAll('branch_', '').replaceAll('image_', '').replaceAll('_','-')
        if(key ==~ '^branch_(.)+') {
            item = string("name": key, "value": G_branches["${nKey}"])
        } else {
            if(key ==~ '^image_(.)+') {
                item = string("name": key, "value": G_images["${nKey}"])
            } else {
                if(key == 'common_version') {
                    item = string("name": key, "value": G_binversion)
                } else {
                    item = string("name": key, "value": value)
                }
            }
        }
        G_params.push(item)
    }
}

def changeParam(sKey, setValue) {
    echo "Changing param for ${sKey} ..."
    
    // change in params
    G_params.eachWithIndex { item, index ->
        if("${item}" ==~ ".*name=${sKey},.*") {
            echo "Removing from G_params: ${item}"
            G_params.remove(index)
        }
    }
    def item = string("name": sKey, "value": setValue)
    echo "Adding to G_params: ${item}"
    G_params.push(item)
    //change in G_images
    def imagesKey = sKey.replaceAll('image_','').replaceAll('_','-').toLowerCase()
    if(G_images.containsKey(imagesKey)) {
        G_images[imagesKey] = setValue
    } else {
        G_images.put(imagesKey, setValue)
    }
    println """Changing param
G_params for ${sKey}: ${G_params}
G_images[${imagesKey}]: ${G_images[imagesKey]}"""
}

def fetchJobData(job_data) {
    job_data.getBuildVariables().eachWithIndex { key, val, index ->
        echo "${key}: ${val}"
        if(key ==~ 'TON_.*' && val) {
            echo "Processing key ${key} ..."
            switch("${key}") {
                case "TON_TYPES":
                    changeParam('image_ton_types', "${val}")
                    break
                case "TON_LABS_TYPES":
                    changeParam('image_ton_labs_types', "${val}")
                    break
                case "TON_BLOCK":
                    changeParam('image_ton_block', "${val}")
                    break
                case "TON_LABS_BLOCK":
                    changeParam('image_ton_labs_block', "${val}")
                    break
                case "TON_BLOCK_JSON":
                    changeParam('image_ton_block_json', "${val}")
                    break
                case "TON_LABS_BLOCK_JSON":
                    changeParam('image_ton_labs_block_json', "${val}")
                    break
                case "TON_VM":
                    changeParam('image_ton_vm', "${val}")
                    break
                case "TON_LABS_VM":
                    changeParam('image_ton_labs_vm', "${val}")
                    break
                case "TON_EXECUTOR":
                    changeParam('image_ton_executor', "${val}")
                    break
                case "TON_LABS_EXECUTOR":
                    changeParam('image_ton_labs_executor', "${val}")
                    break
                case "TON_LABS_ABI":
                    changeParam('image_ton_labs_abi', "${val}")
                    break
                case "TON_SDK":
                    changeParam('image_ton_sdk', "${val}")
                    break
            }
        }
    }
}

pipeline {
    agent {
        node 'master'
    }
    triggers {
        pollSCM('H/15 * * * *') 
    }
    tools {nodejs "Node12.8.0"}
    options {
        buildDiscarder logRotator(artifactDaysToKeepStr: '', artifactNumToKeepStr: '', daysToKeepStr: '', numToKeepStr: '20')
        
        parallelsAlwaysFailFast()
    }
    parameters {
        string(
            name:'common_version',
            defaultValue: '',
            description: 'Common version'
        )
        string(
            name:'branch_ton_types',
            defaultValue: 'master',
            description: 'ton-types branch for dependency test'
        )
        string(
            name:'image_ton_types',
            defaultValue: '',
            description: 'ton-types image name'
        )
        string(
            name:'branch_ton_labs_types',
            defaultValue: '',
            description: 'ton-labs-types branch for dependency test'
        )
        string(
            name:'image_ton_labs_types',
            defaultValue: '',
            description: 'ton-labs-types image name'
        )
        string(
            name:'branch_ton_block',
            defaultValue: 'master',
            description: 'ton-block branch'
        )
        string(
            name:'image_ton_block',
            defaultValue: '',
            description: 'ton-block image name'
        )
        string(
            name:'branch_ton_labs_block',
            defaultValue: '',
            description: 'ton-labs-block branch'
        )
        string(
            name:'image_ton_labs_block',
            defaultValue: '',
            description: 'ton-labs-block image name'
        )
        string(
            name:'branch_ton_vm',
            defaultValue: 'master',
            description: 'ton-vm branch'
        )
        string(
            name:'image_ton_vm',
            defaultValue: '',
            description: 'ton-vm image name'
        )
        string(
            name:'branch_ton_labs_vm',
            defaultValue: '',
            description: 'ton-labs-vm branch'
        )
        string(
            name:'image_ton_labs_vm',
            defaultValue: '',
            description: 'ton-labs-vm image name'
        )
        string(
            name:'branch_ton_labs_abi',
            defaultValue: 'master',
            description: 'ton-labs-abi branch'
        )
        string(
            name:'image_ton_labs_abi',
            defaultValue: '',
            description: 'ton-labs-abi image name'
        )
        string(
            name:'branch_ton_executor',
            defaultValue: 'master',
            description: 'ton-executor branch'
        )
        string(
            name:'image_ton_executor',
            defaultValue: '',
            description: 'ton-executor image name'
        )
        string(
            name:'branch_ton_labs_executor',
            defaultValue: 'master',
            description: 'ton-labs-executor branch'
        )
        string(
            name:'image_ton_labs_executor',
            defaultValue: '',
            description: 'ton-labs-executor image name'
        )
        string(
            name:'branch_tvm_linker',
            defaultValue: 'master',
            description: 'tvm-linker branch'
        )
        string(
            name:'image_tvm_linker',
            defaultValue: '',
            description: 'tvm-linker image name'
        )
        string(
            name:'branch_ton_sdk',
            defaultValue: 'master',
            description: 'ton-sdk branch'
        )
        string(
            name:'image_ton_sdk',
            defaultValue: '',
            description: 'ton-sdk image name'
        )
        string(
            name:'branch_sol2tvm',
            defaultValue: 'master',
            description: 'sol2tvm branch'
        )
    }
    stages {
        stage('Versioning') {
            steps {
                script {
                    if(env.GIT_BRANCH ==~ '[0-9]+.[0-9]+.[0-9]+-rc') {
                        G_binversion = env.GIT_BRANCH.replaceAll('-rc','')
                        echo "Set version from branch: ${G_binversion}"
                    } else {
                        lock('bucket') {
                            withCredentials([file(credentialsId: 'ovh-s3-creds', variable: 'ovhs3')]) {
                                sh """
                                    export AWS_CONFIG_FILE=\$(echo \"\${ovhs3}\")
                                    aws s3 cp s3://sdkbinariestonlabsio/version.json ./version.json
                                """
                            }

                            def folders = """ton_sdk \
ton_client/client \
ton_client/platforms/ton-client-node-js \
ton_client/platforms/ton-client-react-native \
ton_client/platforms/ton-client-web"""
                            if(params.common_version) {
                                G_binversion = sh (script: "node tonVersion.js --set ${params.common_version} ${folders}", returnStdout: true).trim()
                            } else {
                                G_binversion = sh (script: "node tonVersion.js ${folders}", returnStdout: true).trim()
                            }

                            if(!isUpstream() && (GIT_BRANCH == 'master' || GIT_BRANCH ==~ '^PR-[0-9]+' || GIT_BRANCH == "${getVar(G_binversion)}-rc")) {
                                withCredentials([file(credentialsId: 'ovh-s3-creds', variable: 'ovhs3')]) {
                                    sh """
                                        export AWS_CONFIG_FILE=\$(echo \"\${ovhs3}\")
                                        aws s3 cp ./version.json s3://sdkbinariestonlabsio/version.json
                                    """
                                }
                            }
                        }
                    }
                }
            }
        }
        stage('Initialize') {
            steps {
                script {
                    G_gitproject = G_giturl.substring(15,G_giturl.length()-4)
                    G_gitproject_dir = G_gitproject.substring(8, G_gitproject.length())
                    C_TEXT = sh (script: "git show -s --format=%s ${GIT_COMMIT}", \
                        returnStdout: true).trim()
                    C_AUTHOR = sh (script: "git show -s --format=%an ${GIT_COMMIT}", \
                        returnStdout: true).trim()
                    C_COMMITER = sh (script: "git show -s --format=%cn ${GIT_COMMIT}", \
                        returnStdout: true).trim()
                    C_HASH = sh (script: "git show -s --format=%h ${GIT_COMMIT}", \
                        returnStdout: true).trim()
                    C_PROJECT = G_giturl.substring(15,G_giturl.length()-4)
                    C_GITURL = sh (script: "echo ${GIT_URL}",returnStdout: true).trim()
                    C_GITCOMMIT = sh (script: "echo ${GIT_COMMIT}", \
                        returnStdout: true).trim()
                }
                echo """Version: ${getVar(G_binversion)}
Branch: ${GIT_BRANCH}
Possible RC: ${getVar(G_binversion)}-rc"""

                buildParams()
                echo "${G_params}" //debug
            }
        }
        stage('Before stages') {
            when {
                expression {
                    return !isUpstream() && (GIT_BRANCH == 'master' || GIT_BRANCH ==~ '^PR-[0-9]+')
                }
            }
            steps {
                script {
                    def beforeParams = G_params
                    beforeParams.push(string("name": "project_name", "value": "ton-sdk"))
                    beforeParams.push(string("name": "stage", "value": "before"))
                    def job_data = build job: 'Builder/build-flow', parameters: beforeParams
                    fetchJobData(job_data)
                    echo "${G_images}" //debug
                    echo "${G_params}" //debug
                }
            }
        }
        stage('Building...') {
            failFast true
            parallel {
                stage('Parallel stages') {
                    when {
                        expression {
                            return !isUpstream() && (GIT_BRANCH == 'master' || GIT_BRANCH ==~ '^PR-[0-9]+')
                        }
                    }
                    steps {
                        script {
                            def intimeParams = G_params
                            intimeParams.push(string("name": "project_name", "value": "ton-sdk"))
                            intimeParams.push(string("name": "stage", "value": "in_time"))
                            def job_data = build job: 'Builder/build-flow', parameters: intimeParams
                            fetchJobData(job_data)
                        }
                    }
                }
                stage('TON-SDK') {
                    steps {
                        script {
                            def job_data = build job: 'Builder/services/TON-SDK', parameters: G_params
                            def image_ton_sdk = null
                            try {
                                image_ton_sdk = job_data.getBuildVariables()['IMAGE']
                                changeParam('image_ton_sdk', image_ton_sdk)
                            } catch(ex) {
                                echo "No IMAGE variable returned"
                            }
                        }
                    }
                }
            }
        }
        stage('Run integration tests') {
            when { 
                expression {
                    return GIT_BRANCH == "${getVar(G_binversion)}-rc"
                }
            }
            steps {
                script {
                    TON_SDK_BIN_VERSION = GIT_BRANCH.replaceAll("\\.", "_")
                    echo "TON_SDK_BIN_VERSION: ${TON_SDK_BIN_VERSION}"

                    deployPath = ''
                    sh """
                        for it in \$(ls)
                        do 
                            mv \$it \$(echo \$it | sed -E \"s/([0-9]+_[0-9]+_[0-9]+)/\\1-rc/\");
                        done
                    """

                    def params = [
                        [
                            $class: 'StringParameterValue',
                            name: 'TON_SDK_BIN_VERSION',
                            value: "${TON_SDK_BIN_VERSION}"
                        ],
                        [
                            $class: 'BooleanParameterValue',
                            name: 'RUN_TESTS_TON_SURF',
                            value: true
                        ],
                    ] 

                    build job: "Integration/integration-tests/master", parameters: params
                }
            }
        }
        stage('After stages') {
            when {
                expression {
                    return !isUpstream() && (GIT_BRANCH == 'master' || GIT_BRANCH ==~ '^PR-[0-9]+')
                }
            }
            steps {
                script {
                    def afterParams = G_params
                    afterParams.push(string("name": "project_name", "value": "ton-sdk"))
                    afterParams.push(string("name": "stage", "value": "after"))
                    def job_data = build job: 'Builder/build-flow', parameters: afterParams
                    fetchJobData(job_data)
                }
            }
        }
    }
    post {
        always {
            script{
                cleanWs notFailBuild: true
            }
        }
    }
}
