<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post Activities</name>
   <tag></tag>
   <elementGuidId>3fa3d05d-1d06-422a-a0a9-a25c91adff3b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${id},\n  \&quot;title\&quot;: \&quot;${title}\&quot;,\n  \&quot;dueDate\&quot;: \&quot;2026-04-19T09:54:47.022Z\&quot;,\n  \&quot;completed\&quot;: true\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f3f7bad7-86b8-435e-a0b8-60a6b3a7e784</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/v1/Activities</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>6d09d7a1-1bf2-47c2-9658-e028b8ec9621</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'aktivitas 1'</defaultValue>
      <description></description>
      <id>952ee6ee-6ce4-4045-81fe-c2d3c40a9e66</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


//mengambil variabel lokal
def variables = request.getVariables()
def expectedId = variables.get('id')
def expectedTitle = variables.get('title')


// Verifikasi variabel lokal
WS.verifyElementPropertyValue(response, 'id', 1)
WS.verifyElementPropertyValue(response, 'title', 'aktivitas 1')

// Verifikasi respon code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// Verifikasi JSON value check
WS.verifyElementPropertyValue(response, 'completed', true)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
